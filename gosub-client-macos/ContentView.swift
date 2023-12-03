import SwiftUI

struct ContentView: View {
    var content: [RenderItem] = []
    @State private var browserTabs: [BrowserTab] = []
    @State private var urlText = ""
    
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
    
    func deleteTabCallback(tabIndex: Int) {
        for i in tabIndex...self.browserTabs.count - 1 {
            self.browserTabs[i].tabIndex -= 1;
        }
        self.browserTabs.remove(at: tabIndex)
    }
    
    var body: some View {
        VStack {
            Spacer()
            HStack {
                /* BACK BUTTON */
                Button(action: {
                    // TODO:
                }) {
                    Image(systemName: "arrow.left")
                }
                
                /* FORWARD BUTTON */
                Button(action: {
                    // TODO:
                }) {
                    Image(systemName: "arrow.right")
                }
                
                /* REFRESH BUTTON*/
                Button(action: {
                    // TODO:
                }) {
                    Image(systemName: "arrow.clockwise")
                }
                
                /* URL BAR */
                TextField("https://", text: $urlText)
                
                /* SEARCH BUTTON */
                Button(action: {
                    // TODO:
                }) {
                    Image(systemName: "magnifyingglass")
                }
            }.padding(.leading, 10).padding(.trailing, 10)
            Divider()
            HStack {
                ForEach(self.browserTabs, id: \.id) { browserTab in
                    browserTab.show()
                }
                Button(action: {
                    self.browserTabs.append(BrowserTab(tabIndex: self.browserTabs.count, tabName: "New Tab", deleteTabCallback: self.deleteTabCallback))
                }) {
                    Image(systemName: "plus")
                }.background(Color.blue).clipShape(Circle())
                Spacer()
            }.padding(.leading, 5)
            .buttonStyle(.bordered).controlSize(.large)
            ZStack(alignment: .topLeading) {
                Color.white.ignoresSafeArea()
                ForEach(self.content) { renderItem in
                    renderItem.getBody().offset(x: renderItem.x, y: renderItem.y)
                }
            }
        }
    }
}

#Preview {
    ContentView()
}
