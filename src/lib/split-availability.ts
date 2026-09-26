/** Whether the applications half of the split page is a place a swipe can stop at.
 *
 *  On Linux this depends on the VPN mode: in proxy mode there is no per-application
 *  routing to promise, so the applications zone is not in the strip at all and the
 *  swipe meets a wall instead of a tab that goes nowhere.
 *
 *  On Windows there is no mode to ask. The service always runs the TUN and routes by
 *  process -- it is handed the split when the tunnel starts -- so the applications
 *  half always exists.
 */
export function appSplitAvailable(): boolean {
  return true;
}
