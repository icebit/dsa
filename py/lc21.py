class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def mergeLists(h1, h2):
    dummy = ListNode()
    curr = dummy

    while h1 and h2:
        if h1.val < h2.val:
           curr.next = h1
           h1 = h1.next
           curr = curr.next
        else:
            curr.next = h2
            h2 = h2.next
            curr = curr.next

    if h1:
        curr.next = h1
    elif h2:
        curr.next = h2

    return dummy.next
