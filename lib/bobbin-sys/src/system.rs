use bobbin_mcu::mcu::Mcu;

use heap::Heap;
use tick::Tick;
use irq_dispatch::{IrqDispatcher, IrqHandler};
use console::Console;

struct SystemToken;
static mut SYSTEM_TOKEN: Option<SystemToken> = Some(SystemToken);

pub struct System<MCU: Mcu, CLK> {
    mcu: MCU,
    clk: CLK,
    heap: Heap,
    tick: Tick,
    dispatcher: IrqDispatcher<MCU>,
    _private: ()
}

impl<MCU: Mcu, CLK> System<MCU, CLK> {
    pub fn take(mcu: MCU, clk: CLK) -> Self {
        unsafe { while let None = SYSTEM_TOKEN.take() {} }
        let mut heap = Heap::take();
        let tick = Tick::take();
        let irq_handlers: &mut [Option<IrqHandler>] = if let Ok(s) = heap.try_slice(None, 8) {
            s
        } else {
            &mut []
        };
        let dispatcher = IrqDispatcher::init(irq_handlers.as_mut_ptr(), irq_handlers.len());
        System {
            mcu,
            clk,
            heap,
            tick,
            dispatcher,
            _private: (),
        }
    }

    pub fn release(system: Self) -> (MCU, CLK) {
        let System { mcu, clk, heap, tick, dispatcher, _private } = system;
        Tick::release(tick);
        Heap::release(heap);
        IrqDispatcher::release(dispatcher);
        unsafe { SYSTEM_TOKEN = Some(SystemToken) }
        (mcu, clk)
    }

    pub fn mcu(&self) -> &MCU {
        &self.mcu
    }

    pub fn mcu_mut(&mut self) -> &mut MCU {
        &mut self.mcu
    }

    pub fn clk(&self) -> &CLK {
        &self.clk
    }

    pub fn clk_mut(&mut self) -> &mut CLK {
        &mut self.clk
    }

    pub fn heap(&self) -> &Heap {
        &self.heap
    }

    pub fn heap_mut(&mut self) -> &mut Heap {
        &mut self.heap
    }

    pub fn tick(&self) -> &Tick {
        &self.tick
    }

    pub fn tick_mut(&mut self) -> &mut Tick {
        &mut self.tick
    }

    pub fn dispatcher(&self) -> &IrqDispatcher<MCU> {
        &self.dispatcher
    }

    pub fn dispatcher_mut(&mut self) -> &mut IrqDispatcher<MCU> {
        &mut self.dispatcher
    }

    pub fn console(&self) -> Option<&'static Console<'static>> {
        Console::borrow()
    }

}