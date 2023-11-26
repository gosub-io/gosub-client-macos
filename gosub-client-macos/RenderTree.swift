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
                let value = String(cString: render_tree_node_text_value(currentNode))
                let font = String(cString: render_tree_node_text_font(currentNode))
                let font_size = CGFloat(render_tree_node_text_font_size(currentNode))
                let is_bold = render_tree_node_text_bold(currentNode)
                self.renderList.append(TextItem(value: value, font: font, font_size: font_size, is_bold: is_bold))
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
