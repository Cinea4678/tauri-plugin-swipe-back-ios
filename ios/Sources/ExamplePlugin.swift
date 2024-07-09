import SwiftRs
import Tauri
import WebKit


class SwipeBackPlugin: Plugin {
    @objc public override func load(webview: WKWebView) {
        webview.allowsBackForwardNavigationGestures = true
    }
}

@_cdecl("init_plugin_swipe_back_ios")
func initPlugin() -> Plugin {
  return SwipeBackPlugin()
}
