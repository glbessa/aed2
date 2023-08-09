# NOMES: Pedro Porto, Gabriel Bessa

#from tabulate import tabulate

"""
class Graph:
    def __init__(self):
        self.graph = []
        self.nodes = []

    def insertNode(self, node):
        self.nodes.append(node)
        
        for row in self.graph:
            row.append(0)   
        
        self.graph.append([ 0 for i in range(len(self.graph) + 1) ])

    def insertPath(self, sourceIndex, destIndex, weight = 1, directed = True):
        if len(self.graph) < sourceIndex or len(self.graph) < destIndex:
            print("Posições inválidas")
            return
        
        self.graph[sourceIndex][destIndex] = weight

        if not directed:
            self.graph[destIndex][sourceIndex] = weight

    def removePath(self, sourceIndex, destIndex, directed = True):
        if len(self.graph) < sourceIndex or len(self.graph) < destIndex:
            print("Posições inválidas")
            return
        
        self.graph[sourceIndex][destIndex] = 0

        if not directed:
            self.graph[destIndex][sourceIndex] = 0

    def setWeight(self, sourceIndex, destIndex, weight):
        if len(self.graph) < sourceIndex or len(self.graph) < destIndex:
            print("Posições inválidas")
            return
        
        self.graph[sourceIndex][destIndex] = weight
    
    def removeNode(self, index):
        self.nodes.pop(index)

        self.graph.pop(index)

        for row in self.graph:
            row.pop(index)

    def getNode(self, index):
        return self.nodes[index]
    
    def __str__(self):
        string = ""
        for row in self.graph:
            for weight in row:
                string += f" {weight} "
            string += "\n"
        
        string += str(self.nodes)
        return string

    #def __str__(self):
    #   return tabulate(self.graph, headers=self.nodes, showindex=self.nodes, numalign="center", tablefmt='orgtbl')
"""

class Graph:
    def __init__(self):
        self.vertices = []
        self.graph = []

    def insertVertex(self, vertex):
        self.vertices.append(vertex)
        self.graph.append([])

    def removeVertex(self, vertexIndex):
        if vertexIndex > len(self.vertices):
            return

        self.vertices.pop(vertexIndex)
        self.graph.pop(vertexIndex)

        for vertex in self.graph:
            for pos, edge in enumerate(vertex):
                if edge[0] == vertexIndex:
                    vertex.pop(pos)

    def insertEdge(self, srcIndex, destIndex, edgeWeight=1, directed=True):
        if srcIndex > len(self.vertices) or destIndex > len(self.vertices):
            return

        for pos, edge in enumerate(self.graph[srcIndex]):
            if edge[0] == destIndex:
                self.graph[srcIndex][pos] = (destIndex, edgeWeight)
                return

        self.graph[srcIndex].append((destIndex, edgeWeight))

    def removeEdge(self, srcIndex, destIndex):
        for pos, edge in enumerate(self.graph[srcIndex]):
            if edge[0] == destIndex:
                self.graph[srcIndex].pop(pos)

    def getEdgeWeight(self, srcIndex, destIndex):
        for edge in self.graph[srcIndex]:
            if edge[0] == destIndex:
                return edge[1]
        return None

    def __str__(self):
        string = ""
        for posV, vertex in enumerate(self.graph):
            string += f"{self.vertices[posV]}"
            if len(vertex) == 0:
                string += " ->"
            for edge in vertex:
                string += f" -> (Dest: {edge[0]}, Weight: {edge[1]})"
            if posV != len(self.graph) - 1:
                string += "\n"
        return string

if __name__ == '__main__':
    """
    g = Graph()
    g.insertNode("pelotas")
    g.insertNode("canguçu")
    g.insertPath(0, 1, 5, False)
    g.printGraph()
    print(g)
    """
    g = Graph()
    g.insertVertex("Pelotas")
    g.insertVertex("Canguçu")
    g.insertEdge(0, 1, 5, False)
    print(g)