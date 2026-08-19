use std::collections::HashMap;

use anyhow::bail;
use image::RgbaImage;

use crate::wgpu_state::WgpuState;

#[allow(dead_code)]
pub struct Atlas {
    pub(crate) bgl: wgpu::BindGroupLayout,
    pub(crate) bg: wgpu::BindGroup,
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) entries: HashMap<usize, AtlasRegion>,
}

#[allow(dead_code)]
pub(crate) struct URect {
    pub(crate) t: u32,
    pub(crate) l: u32,
    pub(crate) w: u32,
    pub(crate) h: u32,
}

pub(crate) struct AtlasRegion {
    pub(crate) uv: [f32; 4],
    pub(crate) px: URect,
}

fn pack_images(
    images: &Vec<RgbaImage>,
    size: u32,
) -> anyhow::Result<(RgbaImage, HashMap<usize, AtlasRegion>)> {
    let mut pairs: Vec<(usize, u32, u32)> = images
        .iter()
        .enumerate()
        .map(|(k, i)| (k, i.width(), i.height()))
        .collect();
    pairs.sort_by_key(|(_, _, h)| std::cmp::Reverse(*h));

    let mut atlas = RgbaImage::new(size, size);
    let mut regions = HashMap::new();

    let mut current_height: u32 = 0;
    let mut current_width: u32 = 0;
    for (k, w, h) in pairs {
        if current_width + w > size {
            current_height += h;
            if current_height > size {
                bail!(
                    "Combined height of the input images {} is bigger than atlas height {}",
                    current_height,
                    size
                );
            }
            current_width = 0;
        }
        image::imageops::overlay(
            &mut atlas,
            &images[k],
            current_width as i64,
            current_height as i64,
        );
        regions.insert(
            k,
            AtlasRegion {
                uv: [
                    current_width as f32 / size as f32,
                    current_height as f32 / size as f32,
                    (current_width + w) as f32 / size as f32,
                    (current_height + h) as f32 / size as f32,
                ],
                px: URect {
                    t: current_height,
                    l: current_width,
                    w,
                    h,
                },
            },
        );
        current_width += w;
    }

    Ok((atlas, regions))
}

const SIZE_CLASSES: [u32; 8] = [64, 128, 256, 512, 1024, 2048, 4096, 8192];

impl Atlas {
    pub fn new(wgpu_state: &WgpuState, images: &Vec<RgbaImage>) -> Self {
        let (atlas, entries) = SIZE_CLASSES
            .iter()
            .find_map(|size_class| pack_images(images, *size_class).ok())
            .unwrap();
        let (width, height) = (atlas.width(), atlas.height());

        let size = wgpu::Extent3d {
            width,
            height,
            ..Default::default()
        };

        let atlas_texture = wgpu_state.device.create_texture(&wgpu::TextureDescriptor {
            size: size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture atlas"),
            view_formats: &[],
        });

        wgpu_state.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &atlas_texture,
                aspect: wgpu::TextureAspect::All,
                origin: wgpu::Origin3d::ZERO,
                mip_level: 0,
            },
            &atlas,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            size,
        );

        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let atlas_sampler = wgpu_state.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let atlas_bgl =
            wgpu_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Texture atlas bgl"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                            count: None,
                        },
                    ],
                });

        let atlas_bg = wgpu_state
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Texture atlas bg"),
                layout: &atlas_bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&atlas_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&atlas_sampler),
                    },
                ],
            });

        Self {
            texture: atlas_texture,
            view: atlas_view,
            sampler: atlas_sampler,
            bgl: atlas_bgl,
            bg: atlas_bg,
            entries,
        }
    }
}
