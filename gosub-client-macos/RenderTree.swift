import Foundation

public class RenderTree {
    var renderTree: render_tree_t = render_tree_t();
    public var renderList: [RenderItem] = []
    
    init(html: String) {
        render_tree_init(&self.renderTree, html);
        // skip root
        var currentNode: UnsafePointer<node_t>? = render_tree_next(&self.renderTree);
        
        while (true) {
            currentNode = render_tree_next(&self.renderTree);
            if (currentNode == nil) {
                break;
            }
            switch (currentNode.unsafelyUnwrapped.pointee.type) {
            case NODE_TYPE_ROOT: continue
            case NODE_TYPE_TEXT: do {
                let value = String(cString: render_tree_node_text_get_value(currentNode))
                let font = String(cString: render_tree_node_text_get_font(currentNode))
                let fontSize = CGFloat(render_tree_node_text_get_font_size(currentNode))
                let isBold = render_tree_node_text_get_bold(currentNode)
                let x = render_tree_node_get_x(currentNode);
                let y = render_tree_node_get_y(currentNode);
                let textItem = TextItem(value: value, font: font, fontSize: fontSize, isBold: isBold);
                textItem.setXY(x: x, y: y);
                self.renderList.append(textItem)
            }
            default:
                continue
            }
        }
    }
    
    deinit {
        render_tree_free(&self.renderTree);
    }
}
