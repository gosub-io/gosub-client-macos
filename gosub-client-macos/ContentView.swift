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
        /*
         NOTE: below will be the actual code used when we get position
         information from the RenderItems, but as a temporary measure we will
         use VStack
         
         ZStack(alignment: .topLeading) {
             Color.white.ignoresSafeArea()
             ForEach(self.content) { render_item in
                 render_item.getBody().offset(x: 0, y: 0)
             }
         }
         */
        ZStack {
            Color.white.ignoresSafeArea()
            HStack {
                VStack(alignment: .leading) {
                    ForEach(self.content) { render_item in
                        render_item.getBody()
                    }
                    Spacer()
                }
                Spacer()
            }
        }
    }
}

#Preview {
    ContentView()
}
