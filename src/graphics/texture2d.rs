use image;
use cgmath::Vector2;

pub struct Texture2D {
    texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub size: Vector2<u32>
}

impl Texture2D {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn get_layout(device: &wgpu::Device, label: &str) -> wgpu::BindGroupLayout {
        // Defines how the texture will be accessed by the shader, including the sampler.
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true }
                    },
                    count: None
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None
                }
            ],
        })
    }

    pub fn from_png(label: &str, device: &wgpu::Device, queue: &wgpu::Queue, data: &[u8],
                    filter: wgpu::FilterMode) -> Self {
        // Load image data into a variable and adjust
        let texture_image = image::load_from_memory(data).unwrap();
        let texture_data = texture_image.to_rgba8();

        // Get dimensions of image
        let dimensions = texture_data.dimensions();

        Self::from_bytes(label, device, queue, dimensions, &texture_data, filter)
    }

    pub fn from_bytes(label: &str, device: &wgpu::Device, queue: &wgpu::Queue, dimensions: (u32, u32), data: &[u8],
                      filter: wgpu::FilterMode) -> Self {
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1
        };

        // Create texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(&label),
            view_formats: &[]
        });

        // Use the queue to transfer the image data into the texture struct
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture, // Where to store the data
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            &data, // The data itself
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0), // One byte for each r, g, b, a
                rows_per_image: Some(dimensions.1)
            },
            texture_size
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter,
            min_filter: filter,
            mipmap_filter: filter,
            ..Default::default()
        });

        // Define the actual data to be bound
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&label),
            layout: &Self::get_layout(device, label),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view)
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler)
                }
            ]
        });

        Self {
            texture,
            bind_group,
            size: Vector2 { x: dimensions.0, y: dimensions.1 }
        }
    }
}