# Rewrite in rust

class TreeNode:
  def __init__(self, val=0, left=None, right=None):
    self.val = val
    self.left = left
    self.right = right

david_node = TreeNode('David')
perelynn_node = TreeNode('Perelynn', david_node)
mom_node = TreeNode('Sally', perelynn_node, TreeNode('Se', TreeNode('Jennifer')))

# def dfs(node): return [node.val] + dfs(node.left) +  dfs(node.right) if node else []
count = 0
def dfs(node, ct):
  if not node: return
  ct += 1
  if node.val == 'David':
    global count
    count = ct
  dfs(node.left, ct) 
  dfs(node.right, ct)

  

def bfs(node):
  from collections import deque
  q, ans = deque([mom_node]), []
  while q:
    curlv, n = [], len(q)
    for _ in range(n):
      curn = q.popleft()
      curlv.append(curn.val)
      if curn.left: q.append(curn.left)
      if curn.right: q.append(curn.right)
    ans.append(curlv)
  return ans


  
# print(dfs(mom_node))
# print(bfs(mom_node))
dfs(mom_node, 0)
print(count)

# """
#                      Mom
#              /.       |       \
#         Perelynn   Se        Daniel
#            /.         |              
#       David        Jennifer
# """
# '\n             Mom\n             /\n        Perelynn\n           /\n      David\n'
# ['Sally', 'Perelynn', 'David']
# ['Sally', 'Perelynn', 'David', 'Daniel']

# [['Sally'], ['Perelynn', 'Se'], ['David', 'Jennifer']]

