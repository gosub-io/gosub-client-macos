import Foundation
import SwiftUI

public class RenderItem: Identifiable {
    public let id = UUID()
    
    public func getBody() -> AnyView {
        return AnyView(EmptyView())
    }
}
