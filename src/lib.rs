#![no_std]
#![doc = include_str!("../README.md")]

use core::sync::atomic::AtomicU32;

// === Linux/Android ===
#[cfg(all(target_os = "linux", not(target_arch = "wasm32")))]
#[path = "linux.rs"]
mod platform;

// === macOS/iOS/watchOS ===
#[cfg(all(any(target_os = "macos", target_os = "ios", target_os = "watchos"), not(target_arch = "wasm32")))]
#[path = "macos.rs"]
mod platform;

// === Windows ===
#[cfg(all(windows, not(target_arch = "wasm32")))]
#[path = "windows.rs"]
mod platform;

// === FreeBSD ===
#[cfg(all(target_os = "freebsd", not(target_arch = "wasm32")))]
#[path = "freebsd.rs"]
mod platform;

// === WASM stub for Pyodide/Emscripten ===
#[cfg(target_arch = "wasm32")]
mod platform {
    use core::sync::atomic::AtomicU32;
    
    #[inline]
    pub fn wait(_atomic: &AtomicU32, _value: u32) {
        // Pyodide без pthreads не поддерживает блокирующие wait
        // No-op: предполагаем, что wait не вызывается в однопоточном режиме
    }
    
    #[inline]
    pub fn wake_one(_atomic: *const AtomicU32) {
        // ⚠️ ВАЖНО: *const AtomicU32 (сырой указатель), а не &AtomicU32!
        // no-op в однопоточном WASM
    }
    
    #[inline]
    pub fn wake_all(_atomic: *const AtomicU32) {
        // ⚠️ ВАЖНО: *const AtomicU32 (сырой указатель), а не &AtomicU32!
        // no-op в однопоточном WASM
    }
}

// === Публичные функции (делегирование на platform) ===

/// If the value is `value`, wait until woken up.
#[inline]
pub fn wait(atomic: &AtomicU32, value: u32) {
    platform::wait(atomic, value)
}

/// Wake one thread that is waiting on this atomic.
#[inline]
pub fn wake_one(atomic: *const AtomicU32) {
    platform::wake_one(atomic);
}

/// Wake all threads that are waiting on this atomic.
#[inline]
pub fn wake_all(atomic: *const AtomicU32) {
    platform::wake_all(atomic);
}

// ❌ СТРОКА 70 (pub use platform::{...}) ДОЛЖНА БЫТЬ УДАЛЕНА ПОЛНОСТЬЮ!
