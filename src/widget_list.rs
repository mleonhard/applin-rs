#![allow(clippy::many_single_char_names)]
use crate::widget_enum::Widget;

/// This struct converts a tuple of widget builders (`Into<Widget>`) to a vector of widgets.
/// It supports tuples of length 0 through 20.
pub struct WidgetList(pub Vec<Widget>);

impl From<Vec<Widget>> for WidgetList {
    fn from(v: Vec<Widget>) -> Self {
        Self(v)
    }
}

impl<A: Into<Widget>> From<A> for WidgetList {
    fn from(a: A) -> Self {
        WidgetList(vec![a.into()])
    }
}

// From tuples of length 0 through 20.
impl From<()> for WidgetList {
    fn from(_: ()) -> Self {
        WidgetList(vec![])
    }
}
impl<A: Into<Widget>> From<(A,)> for WidgetList {
    fn from((a,): (A,)) -> Self {
        WidgetList(vec![a.into()])
    }
}
impl<A: Into<Widget>, B: Into<Widget>> From<(A, B)> for WidgetList {
    fn from((a, b): (A, B)) -> Self {
        WidgetList(vec![a.into(), b.into()])
    }
}
impl<A: Into<Widget>, B: Into<Widget>, C: Into<Widget>> From<(A, B, C)> for WidgetList {
    fn from((a, b, c): (A, B, C)) -> Self {
        WidgetList(vec![a.into(), b.into(), c.into()])
    }
}
impl<A: Into<Widget>, B: Into<Widget>, C: Into<Widget>, D: Into<Widget>> From<(A, B, C, D)>
    for WidgetList
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        WidgetList(vec![a.into(), b.into(), c.into(), d.into()])
    }
}
impl<A: Into<Widget>, B: Into<Widget>, C: Into<Widget>, D: Into<Widget>, E: Into<Widget>>
    From<(A, B, C, D, E)> for WidgetList
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        WidgetList(vec![a.into(), b.into(), c.into(), d.into(), e.into()])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
    > From<(A, B, C, D, E, F)> for WidgetList
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
    > From<(A, B, C, D, E, F, G)> for WidgetList
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H)> for WidgetList
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I)> for WidgetList
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J)> for WidgetList
{
    fn from((a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K)> for WidgetList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L)> for WidgetList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k, l): (A, B, C, D, E, F, G, H, I, J, K, L)) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M)> for WidgetList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m): (A, B, C, D, E, F, G, H, I, J, K, L, M),
    ) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for WidgetList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n): (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    ) -> Self {
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
        O: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for WidgetList
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
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
        O: Into<Widget>,
        P: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for WidgetList
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
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
        O: Into<Widget>,
        P: Into<Widget>,
        Q: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)> for WidgetList
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
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
        O: Into<Widget>,
        P: Into<Widget>,
        Q: Into<Widget>,
        R: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)> for WidgetList
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
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
            r.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
        O: Into<Widget>,
        P: Into<Widget>,
        Q: Into<Widget>,
        R: Into<Widget>,
        S: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)> for WidgetList
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
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
            r.into(),
            s.into(),
        ])
    }
}
impl<
        A: Into<Widget>,
        B: Into<Widget>,
        C: Into<Widget>,
        D: Into<Widget>,
        E: Into<Widget>,
        F: Into<Widget>,
        G: Into<Widget>,
        H: Into<Widget>,
        I: Into<Widget>,
        J: Into<Widget>,
        K: Into<Widget>,
        L: Into<Widget>,
        M: Into<Widget>,
        N: Into<Widget>,
        O: Into<Widget>,
        P: Into<Widget>,
        Q: Into<Widget>,
        R: Into<Widget>,
        S: Into<Widget>,
        T: Into<Widget>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)> for WidgetList
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
        WidgetList(vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
            q.into(),
            r.into(),
            s.into(),
            t.into(),
        ])
    }
}
