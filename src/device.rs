use std::marker::PhantomData;
use tch::Device;
use typenum::Unsigned;

pub trait TensorDevice {
    const DEVICE: Device;

    fn device(&self) -> Device {
        Self::DEVICE
    }
}

pub struct Cpu;

impl TensorDevice for Cpu {
    const DEVICE: Device = Device::Cpu;
}

pub struct Cuda<I: Unsigned> {
    index: PhantomData<I>,
}

impl<I: Unsigned> TensorDevice for Cuda<I> {
    const DEVICE: Device = Device::Cuda(I::USIZE);
}
