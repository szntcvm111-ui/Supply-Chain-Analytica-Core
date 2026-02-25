// Metamask Mobile UI Balance Refresh Fix
function forceRefreshBalance() {
  const current_state = getWalletState();
  if (current_state.needsUpdate) {
    provider.refresh(); // Fixed: ensuring immediate state sync
  }
}
