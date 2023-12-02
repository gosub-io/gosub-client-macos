import Foundation
import SwiftUI

public class RenderItem: Identifiable {
    public let id = UUID()
    public var x = 0.0;
    public var y = 0.0;
    
    public func setXY(x: CGFloat, y: CGFloat) {
        self.x = x;
        self.y = y;
    }
    
    public func getBody() -> AnyView {
        return AnyView(EmptyView())
    }
}
