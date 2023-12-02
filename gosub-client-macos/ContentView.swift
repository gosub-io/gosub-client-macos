import SwiftUI

struct ContentView: View {
    var content: [RenderItem] = []
    
    init() {
        let html = """
            <html>
                <h1>heading 1</h1>
                <h2>heading 2</h2>
                <h3>heading 3</h3>
                <h4>heading 4</h4>
                <h5>heading 5</h5>
                <h6>heading 6</h6>
                <p>paragraph</p>
            </html>
        """
        let renderTree = RenderTree(html: html)
        self.content = renderTree.renderList
    }
    
    var body: some View {
         
         ZStack(alignment: .topLeading) {
             Color.white.ignoresSafeArea()
             ForEach(self.content) { renderItem in
                 renderItem.getBody().offset(x: renderItem.x, y: renderItem.y)
             }
         }
    }
}

#Preview {
    ContentView()
}
