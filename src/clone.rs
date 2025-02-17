// Copyright (c) 2022-2025 Objectionary.com
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

impl<const N: usize> Clone for Sodg<N> {
    /// Make a clone of the graph.
    fn clone(&self) -> Self {
        Self {
            vertices: self.vertices.clone(),
            branches: self.branches.clone(),
            stores: self.stores.clone(),
            next_v: self.next_v,
        }
    }
}

#[cfg(test)]
use crate::Label;

#[test]
fn makes_a_clone() {
    let mut g: Sodg<16> = Sodg::empty(256);
    g.add(1);
    g.add(42);
    g.bind(1, 42, Label::Alpha(0));
    let c = g.clone();
    assert_eq!(2, c.len());
}

#[test]
fn makes_an_empty_clone() {
    let g: Sodg<16> = Sodg::empty(256);
    let c = g.clone();
    assert_eq!(0, c.len());
}
