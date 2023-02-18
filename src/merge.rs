// Copyright (c) 2022-2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::Sodg;
use anyhow::{anyhow, Result};
use log::debug;
use std::collections::HashMap;

impl Sodg {
    /// Merge another graph into the current one.
    ///
    /// It is expected that both graphs are trees. The `left` vertex is expected
    /// to be the root of the current graph, while the `right` vertex is the root
    /// of the graph being merged into the current one.
    pub fn merge(&mut self, g: &Sodg, left: u32, right: u32) -> Result<()> {
        let mut mapped = HashMap::new();
        let before = self.vertices.len();
        self.merge_rec(g, left, right, &mut mapped)?;
        let merged = mapped.len();
        let scope = g.vertices.len();
        if merged != scope {
            return Err(anyhow!(
                "Just {merged} vertices merged, out of {scope}; maybe the graph is not a tree?"
            ));
        }
        debug!(
            "Merged all {merged} vertices into SODG of {}, making it have {} after the merge",
            before,
            self.vertices.len()
        );
        Ok(())
    }

    /// Merge two trees recursively, ignoring the nodes already `mapped`.
    fn merge_rec(
        &mut self,
        g: &Sodg,
        left: u32,
        right: u32,
        mapped: &mut HashMap<u32, u32>,
    ) -> Result<()> {
        if mapped.contains_key(&right) {
            return Ok(());
        }
        mapped.insert(right, left);
        if g.is_full(right)? {
            self.put(left, g.vertices.get(&right).unwrap().data.clone())?;
        }
        for (a, k, to) in g.kids(right)? {
            let target = if let Some(t) = mapped.get(&to) {
                self.bind(left, *t, format!("{a}/{k}").as_str())?;
                *t
            } else if let Some((t, _)) = self.kid(left, &a) {
                t
            } else {
                let id = self.next_id();
                self.add(id)?;
                self.bind(left, id, &a)?;
                id
            };
            self.merge_rec(g, target, to, mapped)?
        }
        Ok(())
    }
}

#[test]
fn merges_two_graphs() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "bar")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(3, g.vertices.len());
    Ok(())
}

#[test]
fn avoids_simple_duplicates() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "foo")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "foo")?;
    extra.add(2)?;
    extra.bind(1, 2, "bar")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(1, g.kid(0, "foo").unwrap().0);
    Ok(())
}

#[test]
fn merges_singletons() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(13)?;
    let mut extra = Sodg::empty();
    extra.add(13)?;
    g.merge(&extra, 13, 13)?;
    assert_eq!(1, g.vertices.len());
    Ok(())
}

#[test]
fn merges_simple_loop() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.bind(1, 2, "foo")?;
    g.bind(2, 1, "bar")?;
    let extra = g.clone();
    g.merge(&extra, 1, 1)?;
    assert_eq!(extra.vertices.len(), g.vertices.len());
    Ok(())
}

#[test]
fn merges_large_loop() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    g.add(2)?;
    g.add(3)?;
    g.add(4)?;
    g.bind(1, 2, "a")?;
    g.bind(2, 3, "b")?;
    g.bind(3, 4, "c")?;
    g.bind(4, 1, "d")?;
    let extra = g.clone();
    g.merge(&extra, 1, 1)?;
    assert_eq!(extra.vertices.len(), g.vertices.len());
    Ok(())
}

#[cfg(test)]
use crate::Hex;

#[test]
fn merges_data() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.put(1, Hex::from(42))?;
    g.merge(&extra, 1, 1)?;
    assert_eq!(42, g.data(1)?.to_i64()?);
    Ok(())
}

#[test]
fn understands_same_name_kids() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.add(2)?;
    g.bind(1, 2, "x")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "b")?;
    extra.add(2)?;
    extra.bind(1, 2, "x")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(5, g.vertices.len());
    assert_eq!(1, g.kid(0, "a").unwrap().0);
    assert_eq!(2, g.kid(1, "x").unwrap().0);
    Ok(())
}

#[test]
fn merges_into_empty_graph() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(1)?;
    let mut extra = Sodg::empty();
    extra.add(1)?;
    extra.add(2)?;
    extra.add(3)?;
    extra.bind(1, 2, "a")?;
    extra.bind(2, 3, "b")?;
    extra.bind(3, 1, "c")?;
    g.merge(&extra, 1, 1)?;
    assert_eq!(3, g.vertices.len());
    assert_eq!(2, g.kid(1, "a").unwrap().0);
    Ok(())
}

#[test]
fn mixed_injection() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(4)?;
    let mut extra = Sodg::empty();
    extra.add(4)?;
    extra.put(4, Hex::from(4))?;
    extra.add(5)?;
    extra.put(5, Hex::from(5))?;
    extra.bind(4, 5, "b")?;
    g.merge(&extra, 4, 4)?;
    assert_eq!(2, g.vertices.len());
    Ok(())
}

#[test]
fn zero_to_zero() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.bind(1, 0, "back")?;
    g.add(2)?;
    g.bind(0, 2, "b")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "c")?;
    extra.bind(1, 0, "back")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}

#[test]
fn finds_siblings() -> Result<()> {
    let mut g = Sodg::empty();
    g.add(0)?;
    g.add(1)?;
    g.bind(0, 1, "a")?;
    g.add(2)?;
    g.bind(0, 2, "b")?;
    let mut extra = Sodg::empty();
    extra.add(0)?;
    extra.add(1)?;
    extra.bind(0, 1, "b")?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(3, g.vertices.len());
    Ok(())
}

#[cfg(test)]
use crate::Script;

#[test]
fn two_big_graphs() -> Result<()> {
    let mut g = Sodg::empty();
    Script::from_str(
        "ADD(0); ADD(1); BIND(0, 1, foo);
        ADD(2); BIND(0, 1, alpha);
        BIND(1, 0, back);",
    )
    .deploy_to(&mut g)?;
    let mut extra = Sodg::empty();
    Script::from_str("ADD(0); ADD(1); BIND(0, 1, bar); BIND(1, 0, back);").deploy_to(&mut extra)?;
    g.merge(&extra, 0, 0)?;
    assert_eq!(4, g.vertices.len());
    Ok(())
}
