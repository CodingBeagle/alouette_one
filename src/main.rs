use windows::{
    Win32::{
        System::*,
        UI::WindowsAndMessaging::*,
        Foundation::*,
        Graphics::Direct3D::*,
        Graphics::Direct3D11::*,
        Graphics::Dxgi::*,
        Graphics::Dxgi::Common::*,
    }, core::Interface  
};

use std::{mem::{size_of}, os::windows::prelude::OsStrExt};
use std::ptr;
use std::ffi::*;
use core::iter::*;

// OWN MODULES
mod beagle_math;

fn main() {
    println!("Hello, world!");

    unsafe {
        // Retrieve module handle (a module being either a .exe file or DLL) for the .exe file.
        // When GetModuleHandleW is called with "None", it returns a handle for the .exe file.
        let h_instance = LibraryLoader::GetModuleHandleW(None);

        // Create a window class.
        // The window class defines the attributes of a window, like style, icon, cursor, menu, and
        // probably most importantly, the Window Procedure.
        // A Window Procedure MUST BE SET, otherwise "CreateWindow..." will fail.
        // You must register a window class, and then afterwards use that class to create a window.
        let mut window_class_name : Vec<u16> = OsStr::new("mainwindow").encode_wide().chain( once(0) ).collect();

        let mut window_class = WNDCLASSEXW::default();
        window_class.cbSize = size_of::<WNDCLASSEXW>() as u32;
        window_class.style = CS_HREDRAW | CS_VREDRAW;
        window_class.hInstance = h_instance;
        window_class.hCursor = LoadCursorW(h_instance, IDC_ARROW);
        window_class.lpszClassName = PWSTR(window_class_name.as_mut_ptr());
        window_class.lpfnWndProc = Some(wndproc);

        // If RegisterClassExW fails, 0 will be returned.
        if RegisterClassExW(&window_class) == 0 {
            panic!("Failed to register window class.");
        }

        // Create window
        // If successful, the function will return a handle  to the new window.
        // If the function fails, the return value will be zero (null).
        let mut window_title : Vec<u16> = OsStr::new("Alouette One").encode_wide().chain( once(0) ).collect();
        let main_window = CreateWindowExW(
            Default::default(),
            PWSTR(window_class_name.as_mut_ptr()),
            PWSTR(window_title.as_mut_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            0, 0, 800, 600,
            None,
            None,
            h_instance,
            ptr::null_mut()
        );

        if main_window == 0 {
            panic!("Failed to create window!");
        }

        // DirectX Initialization
        // It starts with creating a ID3D11Device and ID3D11DeviceContext.
        // These are the two primary interfaces of DirectX 11, helping us to interface with the GPU.

        // The ID3D11Device is used to check feature support and allocate resources.
        let mut dx_device: Option<ID3D11Device> = None;

        // The ID3D11DeviceContext is used to set render states, bind resources to the graphics
        // pipelines, issue rendering commands, etc...
        let mut dx_device_context: Option<ID3D11DeviceContext> = None;

        // Array of feature levels to support.
        // https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level
        // Basically, in DirectX11, what features of DirectX a video card supports is described
        // In terms of feature levels.
        // A feature level is a well-defined set of GPU functionality.
        // When you do device creation, you attempt to create a device for a certain feature level.
        // If device creation fails, it might be that the feature level you request is not supported
        // by the GPU.
        let requested_feature_levels = [
            D3D_FEATURE_LEVEL_11_0
        ];

        let mut feature_level_result = 0;

        // Attempt to create device and device context
        let device_creation_result = D3D11CreateDevice(
            // Pointer to the video adapter. This is left null, meaning it will pick up
            // The first adapter enumerated by EnumAdapters.
            // The video adapter is the GPU.
            None,
            // The Driver Type.
            // I specify hardware to get hardware accelerated DirectX features.
            D3D_DRIVER_TYPE_HARDWARE,
            // Handle to a DLL that implements a software rasterizer. Only relevant
            // If you choose a software driver.
            None,
            // Runtime layers to enable.
            // https://docs.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-devices-layers
            D3D11_CREATE_DEVICE_DEBUG,
            // Requested feature levels.
            requested_feature_levels.as_ptr(),
            // Number of elements in the requested feature levels array
            requested_feature_levels.len() as u32,
            // You should always specify D3D11_SDK_VERSION for the SDK version parameter.
            D3D11_SDK_VERSION,
            // The return values of the function
            &mut dx_device,
            &mut feature_level_result,
            &mut dx_device_context
        );

        match device_creation_result {
            Ok(()) => println!("DirectX Device Created!"),
            Err(e) => panic!("Failed to create DirectX device: {:?}", e)
        }
        
        if feature_level_result != D3D_FEATURE_LEVEL_11_0 {
            panic!("DirectX 11 support is required, but this device does not support it.");
        }

        let dx_device = dx_device.as_ref().unwrap();
        let dx_device_context = dx_device_context.as_ref().unwrap();

        // Create the swap chain.

        // In order to create a swap chain, we need to call CreateSwapChain on a IDXGIFactory.
        // An IDXGIFactory is used to create objects related to the DXGI technology.
        // The issue is that the IDXGIFactory required is the one which was implicitly used
        // to create the device when calling D3D11CreateDevice, so some calls will have to
        // be made to retrieve that factory.
        let idxgi_device : IDXGIDevice = dx_device.cast().unwrap();
        let idxgi_adapter = idxgi_device.GetAdapter().unwrap();
        let idxgi_factory : IDXGIFactory = idxgi_adapter.GetParent().unwrap();

        // Now that we have obtained the IDXGI factory which was also used to create our device
        // We can create the swapchain using that factory.
        let swap_chain_description = create_swap_chain_description(main_window);
        
        let swap_chain = idxgi_factory.CreateSwapChain(
            dx_device,
            &swap_chain_description
        );

        let swap_chain = match swap_chain {
            Ok(swap_chain) => {
                println!("Swap Chain created!");
                swap_chain},
            Err(e) => panic!("Failed to create swap chain {:?}", e)
        };

        // We need to bind the back buffer of our swap chain to the Output Merger Stage,
        // So that the back buffer can be rendered to by the rendering pipeline.
        // In order to do this, we need to create a Render Target View, which is
        // How Direct3D accessess memory with data used to render the scene.
        // A render target is a resource that can be written to by the output-merger stage
        // At the end of a render pass.
        // A render target should also have a corresponding depth-stencil view.

        // Since SwapEffect of the chain is DXGI_SWAP_EFFECT_DISCARD, we only have access
        // To the first buffer (0)
        let swap_chain_back_buffer : ID3D11Resource = swap_chain.GetBuffer(0).unwrap();
        let back_buffer_render_target_view = dx_device.CreateRenderTargetView(swap_chain_back_buffer, ptr::null()).ok();

        // Before binding the rneder target view, we need a depth-stencil view to go with it.
        // Let's create that now.
        // A depth-stencil buffer is a 2D texture used to store depth information.
        // It's used by the Output Merger Stage to determine which pixels should be visible, and which ones shouldn't.

        // Create the 2D texture which will be used as our depth-stencil buffer.
        // In order to create a 2D texture, we fill out a D3D11_TEXTURE2D_DESC struct.
        let mut depth_buffer_texture_description = D3D11_TEXTURE2D_DESC::default();

        // The width and the height of the texture in Texels.
        // Should be the same size as the back buffer we display in our window.
        depth_buffer_texture_description.Width = 800;
        depth_buffer_texture_description.Height = 600;

        // The number of MipMap levels in the texture.
        // We only need 1 mipmap level in our depth buffer.
        depth_buffer_texture_description.MipLevels = 1;

        // The number of textures in the texture array.
        // We only need one texture for our depth buffer.
        depth_buffer_texture_description.ArraySize = 1;

        // The format of the texture.
        // DXGI_FORMAT_D24_UNORM_S8_UINT = 32-bit-z-buffer format supporting 24 bits for depth and 8 bits for stencil.
        depth_buffer_texture_description.Format = DXGI_FORMAT_D24_UNORM_S8_UINT;

        // We simply use no MSAA right now, as I'm not checking for the supported quality level of my hardware.
        depth_buffer_texture_description.SampleDesc.Count = 1;
        depth_buffer_texture_description.SampleDesc.Quality = 0;

        // Usage describes how the texture should be read from and written to.
        // D3D11_USAGE_DEFAULT is the msot common choice. It describes a texture which requires
        // Read and Write access by the GPU.
        depth_buffer_texture_description.Usage = D3D11_USAGE_DEFAULT;

        // BindFlags is used to identify how a resource should be bound to the pipeline.
        // D3D11_BIND_DEPTH_STENCIL = The texture will be bound as a depth-stencil target for the output-merger stage.
        depth_buffer_texture_description.BindFlags = D3D11_BIND_DEPTH_STENCIL;

        let depth_buffer_texture = dx_device.CreateTexture2D(
            &depth_buffer_texture_description, ptr::null()).unwrap();

        // Now that we have our depth-stencil buffer texture, we need to create a depthStencilView resource,
        // which will be used by the pipeline to actually access the depth buffer data.
        let depth_buffer_view_desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
            Format: depth_buffer_texture_description.Format,
            ViewDimension: D3D11_DSV_DIMENSION_TEXTURE2D,
            Flags: 0, // NOT read only
            // Anonymous is used to specify the type of subresource. In this case a Texture2D.
            Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
                Texture2D: D3D11_TEX2D_DSV { MipSlice: 0 }
            }
        };

        let depth_buffer_view = 
            dx_device.CreateDepthStencilView(&depth_buffer_texture, &depth_buffer_view_desc).unwrap();

        // Bind the back-buffer view and depth buffer view to the Output Merger Stage
        dx_device_context.OMSetRenderTargets(
            1, &back_buffer_render_target_view, &depth_buffer_view);

        // TODO: Exercise - Enumerate through the available outputs (monitors) for an adapter. Use IDXGIAdapter::EnumOutputs.
        // TODO: Exercise - Each output has a lit of supported display modes. For each of them, list width, height, refresh rate, pixel format, etc...

        let mut should_quit = false;

        let mut current_message = MSG::default();

        while !should_quit {
            // PROCESS INPUT
            // PeekMessage will retrieve messages associated with the main window and the thread.
            // I specify Null for hwnd because I want to not only retrieve messages associated with the window,
            // But also with the window's thread. This is so I can als ocatch messages like WM_QUIT.
            // By specifying PM_REMOVE, we remove the message from the queue for processing.
            if PeekMessageW(&mut current_message, None, 0, 0, PM_REMOVE) != false {
                if current_message.message == WM_QUIT {
                    should_quit = true;
                }

                // Translate virtual-key messages into character messages.
                // The character message is posted to the calling thread's message queue, to be read the next time the thread
                // Calls the GetMessage or PeekMessage function.
                // The message will be WM_CHAR, with wParam containing the character code of the key.
                TranslateMessage(&current_message);

                // Dispatch message to the window procedure.
                DispatchMessageW(&current_message);
            } else {
                // GAME LOOP

                // RENDER
                let clear_color = beagle_math::Vector4::new(0.45, 0.6, 0.95, 1.0);

                dx_device_context.ClearRenderTargetView(
                    &back_buffer_render_target_view, &clear_color.as_array()[0]);

                dx_device_context.ClearDepthStencilView(
                    &depth_buffer_view,
                    (D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL) as u32, 
                    1.0, 
                    0);

                if swap_chain.Present(1, 0).is_err() {
                    panic!("Failed to present!");
                }
            }
        }
    }
}

fn create_swap_chain_description(main_window: isize) -> DXGI_SWAP_CHAIN_DESC {
        // A swap chain represents a chain of off screen textures, in the simplest case
        // a back buffer and front buffer. The back buffer is rendered to whilst the
        // front buffer is what is currently being displayed on the monitor.
        // When the back buffer is ready to be rendered, the back buffer and front buffer
        // switch roles, so the newly rendered back buffer becomes the front buffer and is
        // rendered to the screen.
        // This technique is used in order to avoid screen tearing, the visual artifact of the
        // user seeing a frame being drawn before it's done.
        // The act of swapping the front and back buffer is called PRESENTING in DirectX.
        let mut swap_chain_description = DXGI_SWAP_CHAIN_DESC::default();
        
        // Dimensions of the swap chain
        swap_chain_description.BufferDesc.Width = 800;
        swap_chain_description.BufferDesc.Height = 600;

        // Refresh rate of the swap chain
        swap_chain_description.BufferDesc.RefreshRate.Numerator = 60;
        swap_chain_description.BufferDesc.RefreshRate.Denominator = 1;

        // Format of the buffer
        // DXGI_FORMAT_R8G8B8A8_UNORM = Four component, 32-bit unsigned-normalized-integer which
        // Supports 8 bits per channel, including alpha.
        swap_chain_description.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;

        // Scanline ordering is used to specify the method the raster uses to draw the image.
        swap_chain_description.BufferDesc.ScanlineOrdering = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED;

        // Sample description is used to describe multi sampling properties.
        // Count is used to describe the number of multisamples per pixel.
        // Quality is used to describe the quality level. Higher quality = lower performance.
        swap_chain_description.SampleDesc.Count = 1;
        swap_chain_description.SampleDesc.Quality = 0;

        // BufferUsage is used to indicate the surface usage and CPU access options for the back buffer.
        // DXGI_USAGE_RENDER_TARGET_OUTPUT means that we want the back buffer to be used for rendering
        // Output of the graphcis pipeline.
        swap_chain_description.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;

        // TODO: What can this be set to?
        swap_chain_description.BufferCount = 1;

        // OutputWindow is a handle to the output window.
        // This value CANNOT be null.
        swap_chain_description.OutputWindow = main_window;

        // Set the output to windowed mode. This is a fairly important value.
        // If the swap-chain is in windowed mode, the front-buffer is the desktop.
        // If the swap-chain is not in windowed mode, there is a dedicated front buffer.
        // Creating a full-screen swap-chain with an unsupported display mode will cause
        // the display to go black, preventing the end user from seeing anything.
        swap_chain_description.Windowed = BOOL(1);

        // The SwapEffect is used to indicate what to do with the pixels in a display buffer
        // After the PRESENT action has been performed.
        // DXGI_SWAP_EFFECT_DISCARD simply means that the display driver will select the most
        // efficient presentation technique for the swap chain.
        // Also means that the content of the back buffer is discarded after present.
        swap_chain_description.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;

        swap_chain_description
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            // WM_CHAR is a message that is posed after calling TranslateMessage + DispatchMessage.
            // It contains the character encoding of whatever virtual-key was pressed
            // In the message's WPARAM.
            // WM_CHARs will not be generated for non-character keys (like arrow keys, delete, enter, etc...)
            WM_CHAR => {
                println!("Character key was pressed!");
                0
            },
            WM_DESTROY => {
                println!("Destroying window!");
                PostQuitMessage(0);
                0
            },
            _ => DefWindowProcW(window, message, wparam, lparam)
        }
    }
}