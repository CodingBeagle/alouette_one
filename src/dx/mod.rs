/*
    Essentially I am currently designing the system to access the DirectX context as a
    mutable global variable.


    Unsafe? 
    Perhaps. If I knew that I have a multi-threaded application that could mutate it at any point, which I don't right now.
    
    Tightly coupled code when using the global variable?
    Yep. But this is a DirectX interface. It's always going to be what it is, and I have no plan of moving into the land
    of generic Graphics APIs yet. I simply don't know enough about even one to have any clue what can be effectively generalized on a higher abstraction level yet.
    It would essentially be a waste of time for me to try and do this exercise right now.
    So I'd rather start with something that works, and improve it as I go along.

*/

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

pub struct Device {
    pub device: ID3D11Device,
    pub context: ID3D11DeviceContext
}

pub static mut DX: Option<Device> = None;

pub fn initialize_directx() {
    unsafe {
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

        DX = Some(Device {
            device: dx_device.unwrap(),
            context: dx_device_context.unwrap()
        })
    }
}