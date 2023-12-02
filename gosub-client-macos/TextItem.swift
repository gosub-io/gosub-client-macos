import SwiftUI

public final class TextItem : RenderItem {
    
    var value: String
    var font: String
    var fontSize: CGFloat
    var isBold: Bool
    
    init(value: String, font: String, fontSize: CGFloat, isBold: Bool) {
        self.value = value
        self.font = font
        self.fontSize = fontSize
        self.isBold = isBold
    }
    
    public override func getBody() -> AnyView {
        return AnyView(Text(self.value).font(.custom(self.font, size: self.fontSize)).foregroundStyle(.black))
    }
}
