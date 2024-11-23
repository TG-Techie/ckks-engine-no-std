(function() {
    var implementors = Object.fromEntries([["bitvec",[["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.Chunks.html\" title=\"struct bitvec::slice::Chunks\">Chunks</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.ChunksExact.html\" title=\"struct bitvec::slice::ChunksExact\">ChunksExact</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.ChunksExactMut.html\" title=\"struct bitvec::slice::ChunksExactMut\">ChunksExactMut</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.ChunksMut.html\" title=\"struct bitvec::slice::ChunksMut\">ChunksMut</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.Iter.html\" title=\"struct bitvec::slice::Iter\">Iter</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.IterMut.html\" title=\"struct bitvec::slice::IterMut\">IterMut</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.RChunks.html\" title=\"struct bitvec::slice::RChunks\">RChunks</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.RChunksExact.html\" title=\"struct bitvec::slice::RChunksExact\">RChunksExact</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.RChunksExactMut.html\" title=\"struct bitvec::slice::RChunksExactMut\">RChunksExactMut</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.RChunksMut.html\" title=\"struct bitvec::slice::RChunksMut\">RChunksMut</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.Windows.html\" title=\"struct bitvec::slice::Windows\">Windows</a>&lt;'a, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;'a, O, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.RSplit.html\" title=\"struct bitvec::slice::RSplit\">RSplit</a>&lt;'a, O, T, P&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;'a, O, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.RSplitMut.html\" title=\"struct bitvec::slice::RSplitMut\">RSplitMut</a>&lt;'a, O, T, P&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;'a, O, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.Split.html\" title=\"struct bitvec::slice::Split\">Split</a>&lt;'a, O, T, P&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;'a, O, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.SplitMut.html\" title=\"struct bitvec::slice::SplitMut\">SplitMut</a>&lt;'a, O, T, P&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\" title=\"trait core::ops::function::FnMut\">FnMut</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, &amp;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>,</div>"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"enum\" href=\"bitvec/domain/enum.Domain.html\" title=\"enum bitvec::domain::Domain\">Domain</a>&lt;'a, T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;M, O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/ptr/struct.BitPtrRange.html\" title=\"struct bitvec::ptr::BitPtrRange\">BitPtrRange</a>&lt;M, O, T&gt;<div class=\"where\">where\n    M: Mutability,\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.IterOnes.html\" title=\"struct bitvec::slice::IterOnes\">IterOnes</a>&lt;'_, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;O, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/slice/struct.IterZeros.html\" title=\"struct bitvec::slice::IterZeros\">IterZeros</a>&lt;'_, O, T&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    T: <a class=\"trait\" href=\"bitvec/store/trait.BitStore.html\" title=\"trait bitvec::store::BitStore\">BitStore</a>,</div>"],["impl&lt;O, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"bitvec/array/struct.IntoIter.html\" title=\"struct bitvec::array::IntoIter\">IntoIter</a>&lt;O, V&gt;<div class=\"where\">where\n    O: <a class=\"trait\" href=\"bitvec/order/trait.BitOrder.html\" title=\"trait bitvec::order::BitOrder\">BitOrder</a>,\n    V: <a class=\"trait\" href=\"bitvec/view/trait.BitView.html\" title=\"trait bitvec::view::BitView\">BitView</a>,</div>"]]],["memchr",[["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/all/memchr/struct.OneIter.html\" title=\"struct memchr::arch::all::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/all/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::all::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/all/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::all::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.OneIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/avx2/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::x86_64::avx2::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.OneIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::OneIter\">OneIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.ThreeIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::ThreeIter\">ThreeIter</a>&lt;'a, 'h&gt;"],["impl&lt;'a, 'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/arch/x86_64/sse2/memchr/struct.TwoIter.html\" title=\"struct memchr::arch::x86_64::sse2::memchr::TwoIter\">TwoIter</a>&lt;'a, 'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr.html\" title=\"struct memchr::Memchr\">Memchr</a>&lt;'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr2.html\" title=\"struct memchr::Memchr2\">Memchr2</a>&lt;'h&gt;"],["impl&lt;'h&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"memchr/struct.Memchr3.html\" title=\"struct memchr::Memchr3\">Memchr3</a>&lt;'h&gt;"]]],["num_bigint",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"num_bigint/struct.U32Digits.html\" title=\"struct num_bigint::U32Digits\">U32Digits</a>&lt;'_&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"num_bigint/struct.U64Digits.html\" title=\"struct num_bigint::U64Digits\">U64Digits</a>&lt;'_&gt;"]]],["regex",[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SetMatchesIntoIter.html\" title=\"struct regex::bytes::SetMatchesIntoIter\">SetMatchesIntoIter</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/struct.SetMatchesIntoIter.html\" title=\"struct regex::SetMatchesIntoIter\">SetMatchesIntoIter</a>"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/bytes/struct.SetMatchesIter.html\" title=\"struct regex::bytes::SetMatchesIter\">SetMatchesIter</a>&lt;'a&gt;"],["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex/struct.SetMatchesIter.html\" title=\"struct regex::SetMatchesIter\">SetMatchesIter</a>&lt;'a&gt;"]]],["regex_automata",[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"regex_automata/struct.PatternSetIter.html\" title=\"struct regex_automata::PatternSetIter\">PatternSetIter</a>&lt;'a&gt;"]]],["syn",[["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.Iter.html\" title=\"struct syn::punctuated::Iter\">Iter</a>&lt;'a, T&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IterMut.html\" title=\"struct syn::punctuated::IterMut\">IterMut</a>&lt;'a, T&gt;"],["impl&lt;'a, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.Pairs.html\" title=\"struct syn::punctuated::Pairs\">Pairs</a>&lt;'a, T, P&gt;"],["impl&lt;'a, T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.PairsMut.html\" title=\"struct syn::punctuated::PairsMut\">PairsMut</a>&lt;'a, T, P&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IntoIter.html\" title=\"struct syn::punctuated::IntoIter\">IntoIter</a>&lt;T&gt;"],["impl&lt;T, P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/double_ended/trait.DoubleEndedIterator.html\" title=\"trait core::iter::traits::double_ended::DoubleEndedIterator\">DoubleEndedIterator</a> for <a class=\"struct\" href=\"syn/punctuated/struct.IntoPairs.html\" title=\"struct syn::punctuated::IntoPairs\">IntoPairs</a>&lt;T, P&gt;"]]]]);
    if (window.register_implementors) {
        window.register_implementors(implementors);
    } else {
        window.pending_implementors = implementors;
    }
})()
//{"start":57,"fragment_lengths":[14968,4729,727,1502,409,2253]}