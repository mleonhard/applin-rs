use crate::internal::{OptWidgetList, Widget};

/// This struct converts tuples of tuples of widget builders (`Into<WidgetList>`) to a `Vec<Vec<Widget>>`.
/// It supports tuples of length 0 through 10.
pub struct RowGroup(pub Vec<Vec<Option<Widget>>>);

// From tuples of length 0 through 20.
impl From<()> for RowGroup {
    fn from(_: ()) -> Self {
        RowGroup(vec![])
    }
}
impl<A: Into<OptWidgetList>> From<(A,)> for RowGroup {
    fn from((a,): (A,)) -> Self {
        RowGroup(vec![a.into().0])
    }
}
impl<A: Into<OptWidgetList>, B: Into<OptWidgetList>> From<(A, B)> for RowGroup {
    fn from((a, b): (A, B)) -> Self {
        RowGroup(vec![a.into().0, b.into().0])
    }
}
impl<A: Into<OptWidgetList>, B: Into<OptWidgetList>, C: Into<OptWidgetList>> From<(A, B, C)>
    for RowGroup
{
    fn from((a, b, c): (A, B, C)) -> Self {
        RowGroup(vec![a.into().0, b.into().0, c.into().0])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
    > From<(A, B, C, D)> for RowGroup
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        RowGroup(vec![a.into().0, b.into().0, c.into().0, d.into().0])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
    > From<(A, B, C, D, E)> for RowGroup
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F)> for RowGroup
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G)> for RowGroup
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H)> for RowGroup
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I)> for RowGroup
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J)> for RowGroup
{
    fn from((a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K)> for RowGroup
{
    fn from((a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L)> for RowGroup
{
    fn from((a, b, c, d, e, f, g, h, i, j, k, l): (A, B, C, D, E, F, G, H, I, J, K, L)) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m): (A, B, C, D, E, F, G, H, I, J, K, L, M),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n): (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
        ),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
        ),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
        ),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
        R: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
        ),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
            r.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
        R: Into<OptWidgetList>,
        S: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
        ),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
            r.into().0,
            s.into().0,
        ])
    }
}
impl<
        A: Into<OptWidgetList>,
        B: Into<OptWidgetList>,
        C: Into<OptWidgetList>,
        D: Into<OptWidgetList>,
        E: Into<OptWidgetList>,
        F: Into<OptWidgetList>,
        G: Into<OptWidgetList>,
        H: Into<OptWidgetList>,
        I: Into<OptWidgetList>,
        J: Into<OptWidgetList>,
        K: Into<OptWidgetList>,
        L: Into<OptWidgetList>,
        M: Into<OptWidgetList>,
        N: Into<OptWidgetList>,
        O: Into<OptWidgetList>,
        P: Into<OptWidgetList>,
        Q: Into<OptWidgetList>,
        R: Into<OptWidgetList>,
        S: Into<OptWidgetList>,
        T: Into<OptWidgetList>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)> for RowGroup
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t): (
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
        ),
    ) -> Self {
        RowGroup(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
            f.into().0,
            g.into().0,
            h.into().0,
            i.into().0,
            j.into().0,
            k.into().0,
            l.into().0,
            m.into().0,
            n.into().0,
            o.into().0,
            p.into().0,
            q.into().0,
            r.into().0,
            s.into().0,
            t.into().0,
        ])
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GroupedRowTable {
    // TODO: Make all widget fields public.
    row_groups: Vec<Vec<Vec<Option<Widget>>>>,
    spacing: u16,
}
impl GroupedRowTable {
    /// Makes a table widget with grouped rows.
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            row_groups: Vec::new(),
            spacing: 0,
        }
    }

    /// Appends a group of rows.
    #[must_use]
    pub fn with_row_group(mut self, row_group: impl Into<RowGroup>) -> Self {
        self.row_groups.push(row_group.into().0);
        self
    }

    #[must_use]
    pub fn with_spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    #[must_use]
    pub fn to_widget(self) -> Widget {
        Widget::GroupedRowTableVariant {
            row_groups: self.row_groups,
            spacing: self.spacing,
        }
    }
}
impl From<GroupedRowTable> for Widget {
    fn from(src: GroupedRowTable) -> Self {
        src.to_widget()
    }
}
impl From<GroupedRowTable> for Option<Widget> {
    fn from(src: GroupedRowTable) -> Self {
        Some(src.to_widget())
    }
}
