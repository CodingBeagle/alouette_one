use windows::{
    Win32::{
        System::*,
        UI::WindowsAndMessaging::*,
        Foundation::*,
        Graphics::Direct3D::*,
        Graphics::Direct3D11::*,
        Graphics::Dxgi::*,
        Graphics::Dxgi::Common::*,
    }, core::{Interface, DefaultType}  
};

use std::{mem::{size_of, self}, os::windows::prelude::OsStrExt, env, fs, path::PathBuf};
use std::ptr;
use std::ffi::*;
use std::collections::{HashMap};
use core::iter::*;

extern crate base64;

use serde::{Serialize, Deserialize};

// OWN MODULES
mod beagle_math;

#[derive(Debug)]
struct tester {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Default)]
struct Camera {
    position: beagle_math::Vector3,
    orientation: beagle_math::Vector3
}

impl Camera {
    fn view_matrix(&self) -> beagle_math::Mat4 {
        beagle_math::Mat4::translate(&beagle_math::Vector3::new(self.position.x * -1.0, self.position.y * -1.0, self.position.z * -1.0)) 
    }
}

struct VertexConstantBuffer {
    worldViewProjection: beagle_math::Mat4
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GLTF {
    meshes: Vec<Mesh>,
    accessors: Vec<Accessor>,
    buffer_views: Vec<BufferView>,
    buffers: Vec<Buffer>
}

/*
    An Accessor defines a method for retrieving data as typed arrays
    from within a "Buffer View".

    The Accessor will specify things such as the component type, data type,
    the number of elements, etc...
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Accessor {
    // A reference index to the buffer view containing the corresponding data
    buffer_view: u32,
    // The data type of each individual value (component)
    // 5126 = float, 32 bits, 4 bytes
    component_type: u32,
    // Count is the number of elements in the buffer
    count: u32,
    // The type of element that components are described as.
    // VEC3 = 3 components
    #[serde(rename = "type")]
    element_type: String
}

/*
    A "Buffer View" represents a contiguous segment of data
    in a buffer. You can have multiple buffer views into the same
    underlying buffer.
*/
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BufferView {
    // A reference index to an underlying buffer.
    buffer: u32,
    // The amount of bytes in the buffer that this view cares about
    byte_length: u32,
    // The start offset in bytes for this buffer view.
    byte_offset: u32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Buffer {
    byte_length: u32,
    uri: String
}

/*
    Meshes in GLTF represents the data required for GPU draw calls.
*/
#[derive(Serialize, Deserialize, Debug)]
struct Mesh {
    name: String,
    primitives: Vec<Primitive>
}


/*
    Primitives are the actual structures that describes the data
    needed in order to make a GPU draw call for that primitive.
*/
#[derive(Serialize, Deserialize, Debug)]
struct Primitive {
    /*
        Each attribute is a value to an index of an accessor which
        contains the data for the attribute.
    */
    attributes: Attribute,
    /*
        Primitives that are indexed defines this indices property.
        This value is a reference to an accessor containing the corresponding data.
    */
    indices: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
    #[serde(rename = "POSITION")]
    position: u32
}

fn main() {
    unsafe {
        // Path to working directory of executable when running the application
        let current_executable_path = env::current_exe().unwrap();

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

        let path_to_mesh = current_executable_path.parent().unwrap().join("resources\\plane\\plane.gltf");

        //let mesh = load_model(&path_to_mesh, &dx_device);

        let json = fs::read_to_string(path_to_mesh).unwrap();

        let deserialized: GLTF = serde_json::from_str(&json).unwrap();

        let mut box_vertices: [f32; 12] = [
            -5.5,  5.5, 0.0,
             5.5,  5.5, 0.0,
            -5.5, -5.5, 0.0,
             5.5, -5.5, 0.0
        ];

        // https://docs.microsoft.com/en-us/windows/win32/api/d3d11/ns-d3d11-d3d11_buffer_desc
        // D3D11_BUFFER_DESC is used to describe the buffer we want to upload data to
        let mut vertex_buffer_description = D3D11_BUFFER_DESC::default();
        vertex_buffer_description.ByteWidth = (mem::size_of::<f32>() * box_vertices.len()) as u32;
        vertex_buffer_description.Usage = D3D11_USAGE_DEFAULT;
        vertex_buffer_description.BindFlags = D3D11_BIND_VERTEX_BUFFER;

        // D3D11_SUBRESOURCE_DATA is used to supply the data we want to initialize a buffer with
        let mut vertex_buffer_data = D3D11_SUBRESOURCE_DATA::default();
        vertex_buffer_data.pSysMem = box_vertices.as_ptr() as *mut c_void;

        let mut vertex_buffer =
            match dx_device.CreateBuffer(&vertex_buffer_description, &vertex_buffer_data) {
                Ok(buffer) => buffer,
                Err(err) => panic!("Failed to create vertex buffer: {}", err)
            };

        // After we have a vertex buffer, it needs to be bound to an INPUT SLOT, to feed the vertices to the pipeline as input.
        let size_of_vertex_struct = (mem::size_of::<f32>() * 3) as u32;
        let p_offsets = 0;
        
        dx_device_context.IASetVertexBuffers(
            0,
            1,
            &Some(vertex_buffer),
            &size_of_vertex_struct,
            &p_offsets);

        // TODO: Read up on this whole layout object thing again...
        let semantic_name_position = CString::new("POSITION").unwrap();

        let input_element_descriptions = [
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PSTR(semantic_name_position.as_ptr() as *mut u8),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: D3D11_APPEND_ALIGNED_ELEMENT,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0
            }
        ];

        let path_to_vertex_shader = current_executable_path.parent().unwrap().join("resources\\shaders\\compiled-vertex.shader");

        let compiled_vertex_shader_code = fs::read(path_to_vertex_shader).unwrap();

        // CreateInputLayout requires the compiled vertex shader code.
        // This is because it will actually validate the input signature of the VS function to your element descriptions, to see
        // If it fits.
        let input_layout_object = match dx_device.CreateInputLayout(
            input_element_descriptions.as_ptr(),
            1,
            compiled_vertex_shader_code.as_ptr() as *const c_void,
            compiled_vertex_shader_code.len()) {
                Ok(ilo) => ilo,
                Err(err) => panic!("Failed to create InputLayoutObject: {}", err)
            };

        dx_device_context.IASetInputLayout(input_layout_object);

        // We must tell the IA stage how to assemble the vertices into primitives.
        // You do this by specifying a "primitive type" through the Primitive Topology method.
        dx_device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

        // Create an index buffer
        // https://docs.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-resources-buffers-index-how-to
        // An Index Buffer is simply a buffer which contains indices into a vertex buffer. It's used to render primitives more efficiently.
        let mut indices: Vec<i32> = vec![
            0, 1, 2,
            1, 3, 2,
        ];

        let mut index_buffer_description = D3D11_BUFFER_DESC::default();
        index_buffer_description.ByteWidth = (mem::size_of::<i32>() * indices.len()) as u32;
        index_buffer_description.Usage = D3D11_USAGE_DEFAULT;
        index_buffer_description.BindFlags = D3D11_BIND_INDEX_BUFFER;

        let mut index_buffer_data = D3D11_SUBRESOURCE_DATA::default();
        index_buffer_data.pSysMem = indices.as_mut_ptr() as *mut c_void;

        let index_buffer = match dx_device.CreateBuffer(&index_buffer_description, &index_buffer_data) {
            Ok(id) => id,
            Err(err) => panic!("Failed to create index buffer: {}", err)
        };

        dx_device_context.IASetIndexBuffer(index_buffer, DXGI_FORMAT_R32_UINT, 0);

        // Create vertex shader and pixel shader
        let path_to_pixel_shader = current_executable_path.parent().unwrap().join("resources\\shaders\\compiled-pixel.shader");
        let compiled_pixel_shader_code = fs::read(path_to_pixel_shader).unwrap();

        let vertex_shader = match dx_device.CreateVertexShader(
            compiled_vertex_shader_code.as_ptr() as *const c_void, compiled_vertex_shader_code.len(), None) {
                Ok(vs) => vs,
                Err(err) => panic!("Failed to create vertex shader: {}", err)
            };

        let pixel_shader = match dx_device.CreatePixelShader(
            compiled_pixel_shader_code.as_ptr() as *const c_void, compiled_pixel_shader_code.len(), None) {
                Ok(ps) => ps,
                Err(err) => panic!("Failed to create pixel shader: {}", err)
            };

        // A vertex shader must always be active for the pipeline to execute
        dx_device_context.VSSetShader(vertex_shader, ptr::null(), 0);

        dx_device_context.PSSetShader(pixel_shader, ptr::null(), 0);

        // Create Rasterizer state
        // TODO: Definitely read more up on this
        // https://docs.microsoft.com/en-us/windows/win32/api/d3d11/ns-d3d11-d3d11_rasterizer_desc
        let mut rasterizer_description = D3D11_RASTERIZER_DESC::default();
        rasterizer_description.FillMode = D3D11_FILL_SOLID;
        rasterizer_description.CullMode = D3D11_CULL_NONE;
        rasterizer_description.FrontCounterClockwise = BOOL(0);
        rasterizer_description.ScissorEnable = BOOL(0);
        rasterizer_description.DepthClipEnable = BOOL(1);
        rasterizer_description.MultisampleEnable = BOOL(0);

        let rasterizer_state = dx_device.CreateRasterizerState(&rasterizer_description).unwrap();

        dx_device_context.RSSetState(rasterizer_state);

        // The viewport is used by DirectX in the Rasterizer stage, in order to map Normalizerd Device Coordinates Into
        // a 2D surface render target.
        let viewport = D3D11_VIEWPORT {
            Height: 600.0,
            Width: 800.0,
            MinDepth: 0.0,
            MaxDepth: 1.0,
            TopLeftX: 0.0,
            TopLeftY: 0.0
        };

        dx_device_context.RSSetViewports(1, &viewport);

        // Create constant buffer which will be used to upload the world and view matrix to the Vertex shader
        let mut vertex_constant_buffer_description = D3D11_BUFFER_DESC::default();
        vertex_constant_buffer_description.ByteWidth = mem::size_of::<VertexConstantBuffer>() as u32;

        // A constant buffer should be DYNAMIC, as it should be accessible by the GPU and the CPU.
        // Resources with D3D11_USAGE_DYNAMIC cannot be used as destination resources for the UpdateSubresource method.
        // So, if you want to change the content of a D3D11_USAGE_DYNAMIC buffer, use the Map method instead.
        vertex_constant_buffer_description.Usage = D3D11_USAGE_DYNAMIC;

        // We indicate that the buffer should be a constant buffer. These can be used to supply
        // Shader constants to the vertex shader.
        vertex_constant_buffer_description.BindFlags = D3D11_BIND_CONSTANT_BUFFER;

        // We need the CPU to have WRITE ACCESS, so that the CPU can change its contants
        vertex_constant_buffer_description.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;

        let mut world_view_projection_matrix = VertexConstantBuffer {
            worldViewProjection: beagle_math::Mat4::projection((45.0f32).to_radians(), 800.0, 600.0, 0.1, 100.0)
        };

        world_view_projection_matrix.worldViewProjection.tranpose();

        let identity_matrix = D3D11_SUBRESOURCE_DATA {
            pSysMem: &mut world_view_projection_matrix as *mut _ as *mut c_void,
            SysMemPitch: 0,
            SysMemSlicePitch: 0
        };

        let mut vertex_constant_buffer = dx_device.CreateBuffer(&vertex_constant_buffer_description, &identity_matrix).ok();

        if vertex_constant_buffer.is_none() {
            panic!("Failed to create vertex constant buffer!");
        }

        dx_device_context.VSSetConstantBuffers(0, 1, &mut vertex_constant_buffer);

        let mut camera = Camera::default();
        camera.position.z = 0.0;
        camera.position.x = 0.0;

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
                //camera.position.z -= 0.001;
                camera.position.z -= 0.05;

                // RENDER
                let clear_color = beagle_math::Vector4::new(0.45, 0.6, 0.95, 1.0);

                // Update vertex constant buffer for world matrix.
                // The "Map" method retrieves a pointer to the data contained in a subresource (such as our constant buffer), and we can then use
                // That pointer to update its data.
                // When you call the Map method, the GPU will have its access to that subresource denied.
                let lol = vertex_constant_buffer.as_ref().unwrap();
                let mapped_resource = dx_device_context.Map(lol, 0, D3D11_MAP_WRITE_DISCARD, 0);

                if mapped_resource.is_err() {
                    panic!("Failed to retrieve mapped resource for world matrix!");
                }

                let rofl = mapped_resource.unwrap().pData as *mut VertexConstantBuffer;

                // MY MATH LIBRARY CURRENTLY USES ROW-MAJOR CONVENTION, THIS MEANS THAT YOUR TYPICAL P * V * TRSv order becomes vSRT * VIEW * PROJECTION
                (*rofl).worldViewProjection = camera.view_matrix().mul(&beagle_math::Mat4::projection((45.0f32).to_radians(), 800.0, 600.0, 0.1, 100.0));
                
                // My matrices are all designed for being multipled with a ROW vector.
                // Also, I store my matrices in row-major order in memory.
                // By default, HLSL will both READ and PACK matrices in column-major. 
                // So I transpose my matrix so that it will be read correctly as a ROW MAJOR matrix on the shader side.
                (*rofl).worldViewProjection.tranpose();

                // After we're done mapping new data, we have to call Unmap in order to invalidate the pointer to the buffer
                // And reenable the GPU's access to that resource
                dx_device_context.Unmap(lol, 0);

                dx_device_context.ClearRenderTargetView(
                    &back_buffer_render_target_view, &clear_color.as_array()[0]);

                dx_device_context.ClearDepthStencilView(
                    &depth_buffer_view,
                    (D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL) as u32, 
                    1.0, 
                    0);

                dx_device_context.DrawIndexed(indices.len() as u32, 0, 0);

                if swap_chain.Present(1, 0).is_err() {
                    panic!("Failed to present!");
                }
            }
        }
    }
}

enum VertexBufferFormat {
    Vec3Float,
}

struct RawMesh {
    vertex_buffer: ID3D11Buffer,
    vertex_buffer_format: VertexBufferFormat
}

// TODO:
//   Currently my load_model function is heavily mixed up in both interpreting the actual data of the model,
//   As well as creating certain DX buffers for parts of the model (like vertex buffer, index buffer, etc...)
//   Perhaps the function should really only return a structure of all the mesh's data, and then let another part
//   Of the codebase deal with how buffers specifically for DX should be created...
fn load_model(gltf_file_path: &PathBuf, dx_device: &ID3D11Device) -> RawMesh {
    unsafe {
        let gltf_file_content = fs::read_to_string(gltf_file_path).unwrap();
        let gltf: GLTF = serde_json::from_str(&gltf_file_content).unwrap();

        // TODO: Currently only supporting a single mesh
        if (gltf.meshes.len() > 1) {
            panic!("Unsupported amount of meshes.");
        }

        let mut vertex_buffer: Option<ID3D11Buffer> = None;
        let mut vertex_buffer_format: Option<VertexBufferFormat> = None;
    
        for mesh in gltf.meshes {
            // TODO: Currently only supporting simple meshes consisting of 1 primitive
            if mesh.primitives.len() > 1 {
                panic!("Unsupported amount of primitives.");
            }
    
            for primitive in mesh.primitives {
                let mut decoded_buffers : HashMap<u32, Vec<u8>> = HashMap::new();
    
                let vertex_position_accessor_index = primitive.attributes.position;
                let vertex_position_accessor = &gltf.accessors[vertex_position_accessor_index as usize];
    
                let vertex_indices_accessor_index = primitive.indices;
                let vertex_indices_accessor = &gltf.accessors[vertex_indices_accessor_index as usize];
    
                // Vertex Position
                // TODO: I can definitely do a better job at defining these magic literals as descriptive variabels or types
                if vertex_position_accessor.component_type == 5126 
                    && vertex_position_accessor.element_type == "VEC3" {
                        vertex_buffer_format = Some(VertexBufferFormat::Vec3Float);
                } else {
                    panic!("Unsupported combination of component type {} and element type {}", vertex_position_accessor.component_type, vertex_position_accessor.element_type);
                }
    
                let vertex_position_buffer_view = &gltf.buffer_views[vertex_position_accessor.buffer_view as usize];
                let vertex_position_buffer_index = vertex_position_buffer_view.buffer;
                let vertex_position_byte_length = vertex_position_buffer_view.byte_length;
                let vertex_position_byte_offset = vertex_position_buffer_view.byte_offset;            
    
                let decoded_data: &Vec<u8>;
                if decoded_buffers.contains_key(&vertex_position_buffer_index) {
                    decoded_data = decoded_buffers.get(&vertex_position_buffer_index).unwrap();
                } else {
                    let vertex_buffer = &gltf.buffers[vertex_position_buffer_index as usize];
                    decoded_buffers.insert(
                        vertex_position_buffer_index, decode_base64_data_uri(&vertex_buffer.uri));
    
                    decoded_data = decoded_buffers.get(&vertex_position_buffer_index).unwrap();
                }
    
                // TODO: Look... I know this isn't readable, okay? It'll improve!
                let mut vertex_buffer_data: Vec<u8> = vec![0; vertex_position_byte_length as usize];
                vertex_buffer_data.copy_from_slice(&decoded_data[(vertex_position_byte_offset as usize)..((vertex_position_byte_offset + vertex_position_byte_length) as usize)]);
    
                let mut vertex_buffer_description = D3D11_BUFFER_DESC::default();
                vertex_buffer_description.ByteWidth = (mem::size_of::<u8>() * vertex_buffer_data.len()) as u32;
                vertex_buffer_description.Usage = D3D11_USAGE_DEFAULT;
                vertex_buffer_description.BindFlags = D3D11_BIND_VERTEX_BUFFER;
    
                let mut vertex_buffer_subresource = D3D11_SUBRESOURCE_DATA::default();
                vertex_buffer_subresource.pSysMem = vertex_buffer_data.as_ptr() as *mut c_void;
    
                vertex_buffer =
                    match dx_device.CreateBuffer(&vertex_buffer_description, &vertex_buffer_subresource) {
                        Ok(buffer) => Some(buffer),
                        Err(err) => panic!("Failed to create vertex buffer: {}", err)
                    };
            }
        }

        RawMesh {
            vertex_buffer: vertex_buffer.unwrap(),
            vertex_buffer_format: vertex_buffer_format.unwrap()
        }
    }
}

fn decode_base64_data_uri(data_uri: &str) -> Vec<u8> {
    // https://en.wikipedia.org/wiki/Data_URI_scheme
    // data:[<media type>][;base64],<data>
    // TODO:
    //   Currently I'm very naive about my data URI parsing.
    //   Basically I only accept the strict starting format of "data:application/octet-stream;base64"
    if !data_uri.starts_with("data:application/octet-stream;base64") {
        panic!("Unsupported data URI encountered: {}", data_uri);
    }

    let data_in_base64 = data_uri.split_once(",").unwrap().1;

    base64::decode(data_in_base64).unwrap()
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
        // TODO: Getting a DXGI warning using DXGI_SWAP_EFFECT_DISCARD.
        // Apparently this is a legacy swap effect that is superceded by new "flip-models"... gotta read up on this.
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