use renderable::flat_shaded::RenderableMesh;
use windows::{
    Win32::{
        System::*,
        UI::WindowsAndMessaging::*,
        Foundation::*,
        Graphics::Direct3D::*,
        Graphics::Direct3D11::*,
        Graphics::Dxgi::*,
        Graphics::Dxgi::Common::*,
    }, core::{Interface}  
};

use std::{mem::{size_of, self}, os::windows::prelude::OsStrExt, env, fs, ops::Mul};
use std::ptr;
use std::ffi::*;
use std::collections::{HashMap};
use core::iter::*;

// OWN MODULES
mod gltf;
mod gltf2;
mod asset;
mod beagle_math;
mod dx;
mod window;
mod camera;
mod shared;
mod renderable;

// Remember, constant buffers byte width must be multiple of 16
struct VertexConstantBuffer {
    worldViewProjection: beagle_math::Mat4,
    modelMatrix: beagle_math::Mat4,
    cameraPosition: beagle_math::Vector4,
    diffuseColor: beagle_math::Vector4,
    ambientColor: beagle_math::Vector4,
    specularColor: beagle_math::Vector4,
}

pub static mut vertex_normal_shader: Option<ID3D11VertexShader> = None;
pub static mut vertex_normal_shader_input_layout: Option<ID3D11InputLayout> = None;

pub static mut vertex_shader: Option<ID3D11VertexShader> = None;
pub static mut vertex_shader_input_layout: Option<ID3D11InputLayout> = None;

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

        // Get the width and height of the client area of a full-screen window on the primary monitor, in pixels.
        // I will use this to center the game window in the middle of the primary display.
        let desktop_width_in_pixels = GetSystemMetrics(SM_CXFULLSCREEN);
        let desktop_height_in_pixels = GetSystemMetrics(SM_CYFULLSCREEN);

        // If successful, the function will return a handle  to the new window.
        // If the function fails, the return value will be zero (null).
        let mut window_title : Vec<u16> = OsStr::new("Alouette One").encode_wide().chain( once(0) ).collect();
        let main_window = CreateWindowExW(
            Default::default(),
            PWSTR(window_class_name.as_mut_ptr()),
            PWSTR(window_title.as_mut_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            (desktop_width_in_pixels / 2) - (window::WINDOW_WIDTH as i32 / 2), (desktop_height_in_pixels / 2) - (window::WINDOW_HEIGHT as i32 / 2), window::WINDOW_WIDTH as i32, window::WINDOW_HEIGHT as i32,
            None,
            None,
            h_instance,
            ptr::null_mut()
        );

        if main_window == 0 {
            panic!("Failed to create window!");
        }

        dx::initialize_directx();

        let dx_device = &dx::DX.as_ref().unwrap().device;
        let dx_device_context = &dx::DX.as_ref().unwrap().context;

        let mut window_helper = window::Window::default();
        window_helper.lock_cursor_center = true;
        window_helper.hwnd = main_window;
        SetWindowLongPtrA(main_window, GWLP_USERDATA, &window_helper as *const _ as isize);

        // Hide the cursor
        ShowCursor(false);

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
        depth_buffer_texture_description.Width = window::WINDOW_WIDTH;
        depth_buffer_texture_description.Height = window::WINDOW_HEIGHT;

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

        // GLTF2 Testing
        let path_to_mill = current_executable_path.parent().unwrap().join("resources\\mill\\mill.gltf");

        let gltf_file = match gltf2::File::from(path_to_mill) {
            Ok(gltf_file) => gltf_file,
            Err(err) => panic!("{}", err)
        };

        let model = asset::mesh::parse_model(&gltf_file);
        let renderable_data = renderable::flat_shaded::RenderData::from_model(&model);
        let renderable = renderable::flat_shaded::Renderable::from_render_data(renderable_data);

        // TODO: Exercise - Enumerate through the available outputs (monitors) for an adapter. Use IDXGIAdapter::EnumOutputs.
        // TODO: Exercise - Each output has a lit of supported display modes. For each of them, list width, height, refresh rate, pixel format, etc...
        let path_to_vertex_shader = current_executable_path.parent().unwrap().join("resources\\shaders\\shaders\\compiled-vertex.shader");

        let compiled_vertex_shader_code = fs::read(path_to_vertex_shader).unwrap();

        // TODO: Read up on this whole layout object thing again...
        let semantic_name_position = CString::new("POSITION").unwrap();
        let semantic_name_normal = CString::new("NORMAL").unwrap();

        // NOTICE that I am specifying an "Input Slot" for each input element.
        // This is because I am currently using three seperate vertex buffers. One for position, one for color, one for vertex normals.
        // The input slot specifies the index of the vertex array I use for that specific vertex shader parameter.
        // Notice also that the "AlignedByteOffset" is zero, as each elemenent has their own vertex buffer.
        // TODO: You get better performance from interweaved data... so I should probably do that at some point.
        let input_element_descriptions = [
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PSTR(semantic_name_position.as_ptr() as *mut u8),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0
            },
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: PSTR(semantic_name_normal.as_ptr() as *mut u8),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 1,
                AlignedByteOffset: 0,
                InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                InstanceDataStepRate: 0
            }
        ];

        // CreateInputLayout requires the compiled vertex shader code.
        // This is because it will actually validate the input signature of the VS function to your element descriptions, to see
        // If it fits.
        vertex_shader_input_layout = match dx_device.CreateInputLayout(
            input_element_descriptions.as_ptr(),
            2,
            compiled_vertex_shader_code.as_ptr() as *const c_void,
            compiled_vertex_shader_code.len()) {
                Ok(input_layout) => Some(input_layout),
                Err(err) => panic!("Failed to create InputLayoutObject: {}", err)
            };

        dx_device_context.IASetInputLayout(&vertex_shader_input_layout);

        // We must tell the IA stage how to assemble the vertices into primitives.
        // You do this by specifying a "primitive type" through the Primitive Topology method.
        dx_device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

        // Create vertex shader and pixel shader
        let path_to_pixel_shader = current_executable_path.parent().unwrap().join("resources\\shaders\\shaders\\compiled-pixel.shader");
        let compiled_pixel_shader_code = fs::read(path_to_pixel_shader).unwrap();

        vertex_shader = match dx_device.CreateVertexShader(
            compiled_vertex_shader_code.as_ptr() as *const c_void, compiled_vertex_shader_code.len(), None) {
                Ok(vs) => Some(vs),
                Err(err) => panic!("Failed to create vertex shader: {}", err)
            };

        let pixel_shader = match dx_device.CreatePixelShader(
            compiled_pixel_shader_code.as_ptr() as *const c_void, compiled_pixel_shader_code.len(), None) {
                Ok(ps) => ps,
                Err(err) => panic!("Failed to create pixel shader: {}", err)
            };

        // A vertex shader must always be active for the pipeline to execute
        dx_device_context.VSSetShader(&vertex_shader, ptr::null(), 0);
        dx_device_context.PSSetShader(&pixel_shader, ptr::null(), 0);

        // Create Rasterizer state
        // TODO: Definitely read more up on this
        // https://docs.microsoft.com/en-us/windows/win32/api/d3d11/ns-d3d11-d3d11_rasterizer_desc
        // TODO: Make it possible to switch between wireframe and solid mode.
        let mut rasterizer_description = D3D11_RASTERIZER_DESC::default();
        //rasterizer_description.FillMode = D3D11_FILL_WIREFRAME;
        rasterizer_description.FillMode = D3D11_FILL_SOLID;
        //  rasterizer_description.CullMode = D3D11_CULL_NONE;
        rasterizer_description.CullMode = D3D11_CULL_BACK;
        rasterizer_description.FrontCounterClockwise = BOOL(0);
        rasterizer_description.ScissorEnable = BOOL(0);
        rasterizer_description.DepthClipEnable = BOOL(1);
        rasterizer_description.MultisampleEnable = BOOL(0);

        let rasterizer_state = dx_device.CreateRasterizerState(&rasterizer_description).unwrap();

        dx_device_context.RSSetState(rasterizer_state);

        // Prepare shaders for vertex normal rendering
        let path_to_vertex_normals_shader = current_executable_path.parent().unwrap().join("resources\\shaders\\shaders\\compiled-vertex-normals.shader");
        let compiled_vertex_normals_shader_code = fs::read(path_to_vertex_normals_shader).unwrap();

        vertex_normal_shader_input_layout = Some(prepare_vertex_normals_input_layout(&dx_device, &compiled_vertex_normals_shader_code));
        vertex_normal_shader = Some(prepare_vertex_normals_shader(&dx_device, &compiled_vertex_normals_shader_code));

        // The viewport is used by DirectX in the Rasterizer stage, in order to map Normalizerd Device Coordinates Into
        // a 2D surface render target.
        let viewport = D3D11_VIEWPORT {
            Height: window::WINDOW_HEIGHT as f32,
            Width: window::WINDOW_WIDTH as f32,
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

        // We indicate that the buffer should be ???a constant buffer. These can be used to supply
        // Shader constants to the vertex shader.
        vertex_constant_buffer_description.BindFlags = D3D11_BIND_CONSTANT_BUFFER;

        // We need the CPU to have WRITE ACCESS, so that the CPU can change its contants
        vertex_constant_buffer_description.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;

        let mut world_view_projection_matrix = VertexConstantBuffer {
            worldViewProjection: beagle_math::Mat4::projection((45.0f32).to_radians(), window::WINDOW_WIDTH as f32, window::WINDOW_HEIGHT as f32, 0.1, 100.0),
            modelMatrix: beagle_math::Mat4::identity(),
            cameraPosition: beagle_math::Vector4::default(),
            diffuseColor: beagle_math::Vector4::new(1.0, 0.0, 0.0, 0.0),
            ambientColor: beagle_math::Vector4::new(0.15, 0.15, 0.15, 0.0),
            specularColor: beagle_math::Vector4::new(0.5, 0.5, 0.5, 0.0)
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

        let mut should_quit = false;
        let mut current_message = MSG::default();

        let mut drone_camera = camera::FreeFlight::default();
        let mut fps_camera = camera::Fps::default();

        let mut object_position = beagle_math::Vector3::default();

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
                let mut drone_position_delta = beagle_math::Vector3::zero();
                let mut drone_delta_pitch: f32 = 0.0;
                let mut drone_delta_yaw: f32 = 0.0;
                let mut drone_delta_roll: f32 = 0.0;

                if window_helper.is_key_pressed(window::Key::Q) {
                    drone_delta_roll = 0.05;
                }

                if window_helper.is_key_pressed(window::Key::E) {
                    drone_delta_roll = -0.05;
                }

                if window_helper.is_key_pressed(window::Key::D) {
                    drone_position_delta.x = 0.02;
                }

                if window_helper.is_key_pressed(window::Key::A) {
                    drone_position_delta.x = -0.02;
                }

                if window_helper.is_key_pressed(window::Key::W) {
                    drone_position_delta.z = 0.02;
                }

                if window_helper.is_key_pressed(window::Key::S) {
                    drone_position_delta.z = -0.02;
                }

                if window_helper.is_key_pressed(window::Key::Space) {
                    drone_position_delta.y = -0.02;
                }

                if window_helper.is_key_pressed(window::Key::LeftShift) {
                    drone_position_delta.y = 0.02;
                }

                if window_helper.is_key_pressed(window::Key::Escape) {
                    should_quit = true;
                }

                if window_helper.is_key_pressed(window::Key::C) {
                    drone_camera.reset_orientation();
                }
                
                window_helper.update();

                drone_delta_pitch = (window_helper.mouse_move_y as f32) * 0.005;
                drone_delta_yaw = (window_helper.mouse_move_x as f32) * 0.005;

                drone_camera.apply_move(-drone_delta_pitch, drone_delta_yaw, drone_delta_roll, drone_position_delta);
                fps_camera.apply_move(-drone_delta_pitch, drone_delta_yaw, drone_position_delta);

                // RENDER
                let clear_color = beagle_math::Vector4::new(0.45, 0.6, 0.95, 1.0);

                dx_device_context.ClearRenderTargetView(
                    &back_buffer_render_target_view, &clear_color.as_array()[0]);

                dx_device_context.ClearDepthStencilView(
                    &depth_buffer_view,
                    (D3D11_CLEAR_DEPTH | D3D11_CLEAR_STENCIL) as u32, 
                    1.0, 
                    0);

                // Doing something not so pretty here...
                // In my memory arena of renderable meshes, I find the ones that have no parents.
                // These meshes are "root" meshes, meaning they have to be traversed in order to render themselves and their potential children.
                // All other meshes are in some way referenced by others, and will eventually be rendered when traversing the tree
                let mut root_mesh_indices : Vec<u32> = vec!();
                for (index, renderable_mesh) in renderable.renderables.iter().enumerate() {
                    let mut is_child_of_other_mesh = false;

                    // Is this mesh referred to as a child of any other meshes?
                    for (inner_index, inner_renderable_mesh) in renderable.renderables.iter().enumerate() {
                        if inner_index != index 
                            && inner_renderable_mesh.renderable_mesh_data.children.contains(&(index as u16)) {
                                is_child_of_other_mesh = true;
                        }
                    }

                    if !is_child_of_other_mesh {
                        root_mesh_indices.push(index as u32);
                    }
                }

                for root_mesh_index in root_mesh_indices {
                    red(
                        root_mesh_index as u32,
                        &beagle_math::Mat4::identity(),
                        &renderable.renderables,
                        &dx_device_context,
                        vertex_constant_buffer.as_ref().unwrap(),
                        &mut drone_camera
                    );
                }

                if swap_chain.Present(1, 0).is_err() {
                    panic!("Failed to present!");
                }
            }
        }
    }
}

unsafe fn red(
    index: u32,
    mat: &beagle_math::Mat4,
    renderable_meshes: &Vec<RenderableMesh>,
    dx_device_context: &ID3D11DeviceContext,
    constant_buffer: &ID3D11Buffer,
    camera: &mut camera::FreeFlight) -> () {
    let current_renderable_mesh = &renderable_meshes[index as usize];

    // Model Matrix
    // TODO: Need to read up on matrix multiplication order AGAIN... still some detail I'm missing. This is not the order I was expecting myself...
    let model_matrix = beagle_math::Mat4::scale(&current_renderable_mesh.renderable_mesh_data.scale)
        .mul(&current_renderable_mesh.renderable_mesh_data.rotation.to_matrix())
        .mul(&beagle_math::Mat4::translate(&current_renderable_mesh.renderable_mesh_data.translation));

    let combined_matrix = model_matrix.mul(&mat);

    let mapped_resource = dx_device_context.Map(constant_buffer, 0, D3D11_MAP_WRITE_DISCARD, 0);
    if mapped_resource.is_err() {
        panic!("Failed to retrieve mapped resource for world matrix!");
    }

    let constant_vertex_buffer = mapped_resource.unwrap().pData as *mut VertexConstantBuffer;

    let view_matrix = camera.view_matrix();

    let camera_position = camera.get_position();
    (*constant_vertex_buffer).cameraPosition = beagle_math::Vector4::new(camera_position.x, camera_position.y, camera_position.z, 0.0);

    (*constant_vertex_buffer).worldViewProjection = combined_matrix.mul(&view_matrix.mul(&beagle_math::Mat4::projection((60.0f32).to_radians(), window::WINDOW_WIDTH as f32, window::WINDOW_HEIGHT as f32, 0.1, 5000.0)));
    (*constant_vertex_buffer).worldViewProjection.tranpose();

    // TODO: Mat4 should implement the Copy/Clone trait!
    (*constant_vertex_buffer).modelMatrix = beagle_math::Mat4::new(combined_matrix.matrix);
    (*constant_vertex_buffer).modelMatrix.tranpose();

    (*constant_vertex_buffer).diffuseColor = beagle_math::Vector4::new(
        current_renderable_mesh.renderable_mesh_data.material.diffuse_color.x,
        current_renderable_mesh.renderable_mesh_data.material.diffuse_color.y,
        current_renderable_mesh.renderable_mesh_data.material.diffuse_color.z,
        0.0
    );

    (*constant_vertex_buffer).ambientColor = beagle_math::Vector4::new(
        current_renderable_mesh.renderable_mesh_data.material.ambient_color.x,
        current_renderable_mesh.renderable_mesh_data.material.ambient_color.y,
        current_renderable_mesh.renderable_mesh_data.material.ambient_color.z,
        0.0);

    (*constant_vertex_buffer).specularColor = beagle_math::Vector4::new(
        current_renderable_mesh.renderable_mesh_data.material.specular_color.x,
        current_renderable_mesh.renderable_mesh_data.material.specular_color.y,
        current_renderable_mesh.renderable_mesh_data.material.specular_color.z,
        0.0);

    // After we're done mapping new data, we have to call Unmap in order to invalidate the pointer to the buffer
    // And reenable the GPU's access to that resource
    dx_device_context.Unmap(constant_buffer, 0);

    dx_device_context.IASetVertexBuffers(
        0,
        2,
        ([
            Some(current_renderable_mesh.vertex_buffer.clone()),
            Some(current_renderable_mesh.normals_buffer.clone())
        ]).as_ptr(),
        ([
            (mem::size_of::<beagle_math::Vector3>()) as u32,
            (mem::size_of::<beagle_math::Vector3>()) as u32
        ]).as_ptr(),
        ([
            0,
            0
        ]).as_ptr()
    );

    dx_device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    dx_device_context.VSSetShader(&vertex_shader, ptr::null(), 0);
    dx_device_context.IASetInputLayout(&vertex_shader_input_layout);
    dx_device_context.Draw(current_renderable_mesh.renderable_mesh_data.vertex_positions.len() as u32, 0);

    dx_device_context.IASetVertexBuffers(
        0, 
        1,
       ([
           Some(current_renderable_mesh.debug_vertex_normals_buffer.clone())
           ]).as_ptr(),
        [(mem::size_of::<beagle_math::Vector3>() as u32)].as_ptr(),
        [0].as_ptr());
    dx_device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_LINELIST);
    dx_device_context.VSSetShader(&vertex_normal_shader, ptr::null(), 0);
    dx_device_context.IASetInputLayout(&vertex_normal_shader_input_layout);
    dx_device_context.Draw(current_renderable_mesh.renderable_mesh_data.debug_vertex_normals.len() as u32, 0);

    if current_renderable_mesh.renderable_mesh_data.children.len() > 0 {
        for child_index in &current_renderable_mesh.renderable_mesh_data.children {
            red(
                *child_index as u32,
                &combined_matrix,
                renderable_meshes,
                dx_device_context,
                constant_buffer,
                camera
            )
        }
    }
}

fn prepare_vertex_normals_shader(dx_device: &ID3D11Device, compiled_shader_code: &Vec<u8>) -> ID3D11VertexShader {
    unsafe {
        match dx_device.CreateVertexShader(
            compiled_shader_code.as_ptr() as *const c_void,
            compiled_shader_code.len(),
            None) {
                Ok(vs) => vs,
                Err(err) => panic!("Failed to create vertex shader: {}", err)
            }
    }
}

fn prepare_vertex_normals_input_layout(dx_device: &ID3D11Device, compiled_vertex_shader_code: &Vec<u8>) -> ID3D11InputLayout {
    unsafe {
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

        match dx_device.CreateInputLayout(
            input_element_descriptions.as_ptr(),
            1,
            compiled_vertex_shader_code.as_ptr() as *const c_void,
            compiled_vertex_shader_code.len()) {
                Ok(ilo) => ilo,
                Err(err) => panic!("Failed to create input layout object for normals shader: {}", err)
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
        swap_chain_description.BufferDesc.Width = window::WINDOW_WIDTH;
        swap_chain_description.BufferDesc.Height = window::WINDOW_HEIGHT;

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
        let window_helper = GetWindowLongPtrA(window, GWLP_USERDATA) as *mut window::Window;

        match message {
            // WM_CHAR is a message that is posted after calling TranslateMessage + DispatchMessage.
            // It contains the character encoding of whatever virtual-key was pressed in the message's WPARAM.
            // WM_CHARs will not be generated for non-character keys (like arrow keys, delete, enter, etc...)
            WM_CHAR => {
                0
            },
            // WM_KEYDOWN is posted to the window when a nonsystem key is pressed.
            // WPARAM wil contain the virtual-key code of the nonsystem key.
            WM_KEYDOWN => {
                let mapped_key = window::map_to_key(wparam as i32);
                // TODO: Could probably do a better job of encapsulating the hashset in the "Window" struct, and instead expose
                // A method to register and unregister key entries.
                window_helper.as_mut().unwrap().current_keyboard_state.insert(mapped_key);
                0
            },
            // WM_KEYUP is posted to the window when a nonsystem key is released.
            // WPARAM will contain the virtual-key code of the nonsystem key.
            WM_KEYUP => {
                let mapped_key = window::map_to_key(wparam as i32);
                window_helper.as_mut().unwrap().current_keyboard_state.remove(&mapped_key);
                0
            },
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            },
            _ => DefWindowProcW(window, message, wparam, lparam)
        }
    }
}