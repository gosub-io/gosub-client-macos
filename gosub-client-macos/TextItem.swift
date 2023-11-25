import SwiftUI

public final class TextItem : RenderItem {
    
    var value: String
    var font: String
    var font_size: CGFloat
    var is_bold: Bool
    
    init(value: String, font: String, font_size: CGFloat, is_bold: Bool) {
        self.value = value
        self.font = font
        self.font_size = font_size
        self.is_bold = is_bold
    }
    
    public override func getBody() -> AnyView {
        return AnyView(Text(self.value).font(.custom(self.font, size: self.font_size)).foregroundStyle(.black))
    }
}
