use std::marker::PhantomData;

pub(crate) struct Buffer<T: bytemuck::Pod + bytemuck::Zeroable> {
    buf: wgpu::Buffer,
    cap: u64,
    len: u64,
    label: String,
    usage: wgpu::BufferUsages,
    pd: PhantomData<T>,
}

const BLOCK_SIZE: u64 = 1024;

const fn align_up(x: u64, a: u64) -> u64 {
    assert!(a.is_power_of_two());
    (x + a - 1) & !(a - 1)
}

impl<T: bytemuck::Pod + bytemuck::Zeroable> Buffer<T> {
    pub(crate) fn new(usage: wgpu::BufferUsages, device: &wgpu::Device, label: &str) -> Self {
        let usage = usage | wgpu::BufferUsages::COPY_DST;
        Self {
            buf: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                mapped_at_creation: false,
                size: std::mem::size_of::<T>() as u64 * BLOCK_SIZE,
                usage,
            }),
            usage,
            len: 0,
            cap: BLOCK_SIZE,
            label: label.into(),
            pd: PhantomData,
        }
    }

    pub(crate) fn write(&mut self, data: &[T], queue: &wgpu::Queue, device: &wgpu::Device) {
        let size = data.len() as u64;
        self.len = size;
        if size > self.cap {
            let cap = align_up(size, BLOCK_SIZE);
            self.cap = cap;
            self.buf = device.create_buffer(&wgpu::BufferDescriptor {
                size: std::mem::size_of::<T>() as u64 * cap,
                mapped_at_creation: false,
                label: Some(&self.label),
                usage: self.usage,
            })
        }

        queue.write_buffer(&self.buf, 0, bytemuck::cast_slice(data));
    }

    pub(crate) fn length(&self) -> u64 {
        self.len
    }

    #[allow(dead_code)]
    pub(crate) fn capacity(&self) -> u64 {
        self.cap
    }

    pub(crate) fn slice(&self) -> wgpu::BufferSlice<'_> {
        self.buf
            .slice(..(self.length() * std::mem::size_of::<T>() as u64))
    }
}
