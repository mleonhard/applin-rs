use crate::widgets::Widget;
use serde_json::Value;

pub struct WidgetList(pub Vec<Value>);
impl<A: Into<Widget>> From<(A,)> for WidgetList {
    fn from((a,): (A,)) -> Self {
        WidgetList(vec![a.into().0])
    }
}
impl<A: Into<Widget>, B: Into<Widget>> From<(A, B)> for WidgetList {
    fn from((a, b): (A, B)) -> Self {
        WidgetList(vec![a.into().0, b.into().0])
    }
}
impl<A: Into<Widget>, B: Into<Widget>, C: Into<Widget>> From<(A, B, C)> for WidgetList {
    fn from((a, b, c): (A, B, C)) -> Self {
        WidgetList(vec![a.into().0, b.into().0, c.into().0])
    }
}
impl<A: Into<Widget>, B: Into<Widget>, C: Into<Widget>, D: Into<Widget>> From<(A, B, C, D)>
    for WidgetList
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        WidgetList(vec![a.into().0, b.into().0, c.into().0, d.into().0])
    }
}
impl<A: Into<Widget>, B: Into<Widget>, C: Into<Widget>, D: Into<Widget>, E: Into<Widget>>
    From<(A, B, C, D, E)> for WidgetList
{
    fn from((a, b, c, d, e): (A, B, C, D, E)) -> Self {
        WidgetList(vec![
            a.into().0,
            b.into().0,
            c.into().0,
            d.into().0,
            e.into().0,
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
    > From<(A, B, C, D, E, F)> for WidgetList
{
    fn from((a, b, c, d, e, f): (A, B, C, D, E, F)) -> Self {
        WidgetList(vec![
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
