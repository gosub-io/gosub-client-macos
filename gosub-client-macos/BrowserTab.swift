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
    public var selectTabCallback: (Int, [RenderItem]) -> ()
    public var deleteTabCallback: (Int) -> ()
    public var tabIndex: Int
    private let tabName: String
    private var renderTree: RenderTree = RenderTree(html: "")
    private var isSelected: Bool = false
    
    public init(tabIndex: Int, tabName: String, selectTabCallback: @escaping (Int, [RenderItem]) -> (), deleteTabCallback: @escaping (Int) -> ()) {
        self.tabIndex = tabIndex
        self.tabName = tabName
        self.selectTabCallback = selectTabCallback
        self.deleteTabCallback = deleteTabCallback
    }
    
    public func renderHTML(html: String) {
        self.renderTree = RenderTree(html: html);
    }
    
    public func show() -> AnyView {
        return AnyView(Button(action: {
            self.selectTabCallback(self.tabIndex, self.renderTree.renderList)
        }) {
            Text(self.tabName).padding(5)
            Button(action: {
                self.deleteTabCallback(self.tabIndex)
            }) {
                Image(systemName: "xmark")
            }.buttonStyle(.bordered).background(Color.red).controlSize(.small).padding(3).clipShape(Circle())
        }.background(self.isSelected ? Color.blue : Color.clear).clipShape(RoundedRectangle(cornerRadius: 10.0)));
    }
    
    public func setSelected(isSelected: Bool) {
        self.isSelected = isSelected;
    }
}
