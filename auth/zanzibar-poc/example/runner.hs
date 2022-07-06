-- Example runner file which defines some Zanzibar Relations and
-- example relationship tuples

import qualified Zanzibar.Rewrite as Re
import qualified Zanzibar.Model as M
import qualified Zanzibar

owner = M.Relation "owner" (M.Rewrite M.Union [])
editor = M.Relation "editor" (M.Rewrite M.Union [M.ComputeUserset "owner"])
viewer = M.Relation "viewer" (M.Rewrite M.Union [M.ComputeUserset "editor"])
rels = [owner, editor, viewer]


relationships 
    = [
    (("group", "admin"), "member", M.UID "Bob"),
    (("group", "user"), "member", M.UID "Alice"),
    (("doc", "private"), "owner", M.Userset (("group", "admin"), "member")),
    (("doc", "private"), "viewer", M.Userset (("group", "user"), "member")),
    (("doc", "readme"), "viewer", M.All)
    ]

-- Zanzibar.check relationships rels ("doc", "readme") "editor" "Bob"
check = Zanzibar.check relationships rels
