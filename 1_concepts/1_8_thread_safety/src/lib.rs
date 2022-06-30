use std::{
    cell::Cell,
    rc::Rc,
    sync::{atomic::AtomicI32, Arc, MutexGuard},
};

pub fn requires_sync<T: Sync>(_: &T) {}
pub fn requires_send<T: Send>(_: &T) {}

/// `OnlySync`
///
/// ```
/// fn ok(only_sync: step_1_8::OnlySync<'_>) {
///     step_1_8::requires_sync(&only_sync);
/// }
/// ```
/// ```compile_fail,0277
/// fn err(only_sync: step_1_8::OnlySync<'_>) {
///     step_1_8::requires_send(&only_sync);
/// }
/// ```
pub struct OnlySync<'a>(MutexGuard<'a, i32>);

/// `OnlySend`
///
/// ```compile_fail,0277
/// fn ok(only_send: step_1_8::OnlySend) {
///     step_1_8::requires_sync(&only_send);
/// }
/// ```
/// ```
/// fn err(only_send: step_1_8::OnlySend) {
///     step_1_8::requires_send(&only_send);
/// }
/// ```
pub struct OnlySend(Cell<i32>);

/// `SyncAndSend`
///
/// ```
/// fn ok(sync_and_send: step_1_8::SyncAndSend) {
///     step_1_8::requires_sync(&sync_and_send);
///     step_1_8::requires_send(&sync_and_send);
/// }
/// ```
pub struct SyncAndSend(AtomicI32);

/// `NotSyncNotSend`
///
/// ```compile_fail,0277
/// fn err(not_sync_not_send: step_1_8::NotSyncNotSend) {
///     step_1_8::requires_sync(&not_sync_not_send);
///     step_1_8::requires_send(&not_sync_not_send);
/// }
/// ```
pub struct NotSyncNotSend(Rc<i32>);
