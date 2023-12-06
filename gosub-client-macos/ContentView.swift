import SwiftUI

struct ContentView: View {
    @State private var content: [RenderItem] = []
    @State private var browserTabs: [BrowserTab] = []
    @State private var urlText = ""
    
    func selectTabCallback(renderList: [RenderItem]) {
        self.content = renderList;
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
                    let newTab = BrowserTab(tabIndex: self.browserTabs.count, tabName: "New Tab", selectTabCallback: self.selectTabCallback, deleteTabCallback: self.deleteTabCallback);
                    newTab.renderHTML(html: "<html><h1>tab " + String(self.browserTabs.count) + "</h1></html>");
                    self.browserTabs.append(newTab)
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
