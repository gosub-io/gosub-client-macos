//
//  BrowserTab.swift
//  gosub-client-macos
//
//  Created by Zachary Weaver on 12/2/23.
//

import Foundation
import SwiftUI

public class BrowserTab : Identifiable {
    public var id = UUID()
    public var deleteTabCallback: (Int) -> ()
    public var tabIndex: Int
    private let tabName: String
    
    public init(tabIndex: Int, tabName: String, deleteTabCallback: @escaping (Int) -> ()) {
        self.tabIndex = tabIndex
        self.tabName = tabName
        self.deleteTabCallback = deleteTabCallback
    }
    
    public func show() -> AnyView {
        return AnyView(Button(action: {
            print("TAB INDEX: ", self.tabIndex)
            // TODO:
        }) {
            Text(self.tabName).padding(5)
            Button(action: {
                self.deleteTabCallback(self.tabIndex)
            }) {
                Image(systemName: "xmark")
            }.buttonStyle(.bordered).background(Color.red).controlSize(.small).padding(3).clipShape(Circle())
        });
    }
}
