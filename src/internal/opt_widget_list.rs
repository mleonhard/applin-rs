#![allow(clippy::many_single_char_names)]
use crate::internal::Widget;

/// This struct converts a tuple `Into<Option<Widget>>` to a `Vec<Option<Widget>>`.
/// It supports tuples of length 0 through 20.
pub struct OptWidgetList(pub Vec<Option<Widget>>);

impl From<Vec<Widget>> for OptWidgetList {
    fn from(v: Vec<Widget>) -> Self {
        Self(v.into_iter().map(|w| Some(w)).collect())
    }
}

impl From<Vec<Option<Widget>>> for OptWidgetList {
    fn from(v: Vec<Option<Widget>>) -> Self {
        Self(v)
    }
}

// From tuples of length 0 through 20.
impl From<()> for OptWidgetList {
    fn from(_: ()) -> Self {
        OptWidgetList(vec![])
    }
}
impl<A: Into<Option<Widget>>> From<(A,)> for OptWidgetList {
    fn from((a,): (A,)) -> Self {
        OptWidgetList(vec![a.into()])
    }
}
impl<A: Into<Option<Widget>>, B: Into<Option<Widget>>> From<(A, B)> for OptWidgetList {
    fn from((a, b): (A, B)) -> Self {
        OptWidgetList(vec![a.into(), b.into()])
    }
}
impl<A: Into<Option<Widget>>, B: Into<Option<Widget>>, C: Into<Option<Widget>>> From<(A, B, C)>
    for OptWidgetList
{
    fn from((a, b, c): (A, B, C)) -> Self {
        OptWidgetList(vec![a.into(), b.into(), c.into()])
    }
}
impl<
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
    > From<(A, B, C, D)> for OptWidgetList
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        OptWidgetList(vec![a.into(), b.into(), c.into(), d.into()])
    }
}
impl<
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
    > From<(A, B, C, D, E)> for OptWidgetList
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        OptWidgetList(vec![a.into(), b.into(), c.into(), d.into(), e.into()])
    }
}
impl<
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F)> for OptWidgetList
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G)> for OptWidgetList
{
    fn from((a, b, c, d, e, f, g): (A, B, C, D, E, F, G)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H)> for OptWidgetList
{
    fn from((a, b, c, d, e, f, g, h): (A, B, C, D, E, F, G, H)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I)> for OptWidgetList
{
    fn from((a, b, c, d, e, f, g, h, i): (A, B, C, D, E, F, G, H, I)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J)> for OptWidgetList
{
    fn from((a, b, c, d, e, f, g, h, i, j): (A, B, C, D, E, F, G, H, I, J)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K)> for OptWidgetList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k): (A, B, C, D, E, F, G, H, I, J, K)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L)> for OptWidgetList
{
    fn from((a, b, c, d, e, f, g, h, i, j, k, l): (A, B, C, D, E, F, G, H, I, J, K, L)) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M)> for OptWidgetList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m): (A, B, C, D, E, F, G, H, I, J, K, L, M),
    ) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N)> for OptWidgetList
{
    fn from(
        (a, b, c, d, e, f, g, h, i, j, k, l, m, n): (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    ) -> Self {
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
        O: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)> for OptWidgetList
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
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
        O: Into<Option<Widget>>,
        P: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)> for OptWidgetList
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
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
        O: Into<Option<Widget>>,
        P: Into<Option<Widget>>,
        Q: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q)> for OptWidgetList
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
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
        O: Into<Option<Widget>>,
        P: Into<Option<Widget>>,
        Q: Into<Option<Widget>>,
        R: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R)> for OptWidgetList
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
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
        O: Into<Option<Widget>>,
        P: Into<Option<Widget>>,
        Q: Into<Option<Widget>>,
        R: Into<Option<Widget>>,
        S: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S)> for OptWidgetList
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
        OptWidgetList(vec![
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
        A: Into<Option<Widget>>,
        B: Into<Option<Widget>>,
        C: Into<Option<Widget>>,
        D: Into<Option<Widget>>,
        E: Into<Option<Widget>>,
        F: Into<Option<Widget>>,
        G: Into<Option<Widget>>,
        H: Into<Option<Widget>>,
        I: Into<Option<Widget>>,
        J: Into<Option<Widget>>,
        K: Into<Option<Widget>>,
        L: Into<Option<Widget>>,
        M: Into<Option<Widget>>,
        N: Into<Option<Widget>>,
        O: Into<Option<Widget>>,
        P: Into<Option<Widget>>,
        Q: Into<Option<Widget>>,
        R: Into<Option<Widget>>,
        S: Into<Option<Widget>>,
        T: Into<Option<Widget>>,
    > From<(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T)> for OptWidgetList
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
        OptWidgetList(vec![
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
