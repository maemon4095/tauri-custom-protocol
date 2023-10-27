# NODE: custom-protocol restriction

どうやらWebKitの制限でLinuxではカスタムプロトコルが正しく動かないらしい．(https://github.com/tauri-apps/wry/issues/666)

加えて，Access Controlもおかしそう？ カスタムプロトコルがはじかれる．

WebKitには2023/02/27に修正のPRがされている模様(https://github.com/WebKit/WebKit/pull/10714)．

近いアプデで修正されそう．
