(function() {var implementors = {};
implementors["chrono"] = [{"text":"impl Sub&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl Sub&lt;FixedOffset&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;FixedOffset&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;FixedOffset&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl Sub&lt;NaiveDate&gt; for NaiveDate","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;NaiveDateTime&gt; for NaiveDateTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;NaiveTime&gt; for NaiveTime","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;Duration&gt; for Date&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;Date&lt;Tz&gt;&gt; for Date&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;Duration&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Tz:&nbsp;TimeZone&gt; Sub&lt;DateTime&lt;Tz&gt;&gt; for DateTime&lt;Tz&gt;","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;T, S&gt; Sub&lt;&amp;'_ HashSet&lt;T, S&gt;&gt; for &amp;HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;T, S1, S2&gt; Sub&lt;&amp;'_ IndexSet&lt;T, S2&gt;&gt; for &amp;IndexSet&lt;T, S1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S1: BuildHasher + Default,<br>&nbsp;&nbsp;&nbsp;&nbsp;S2: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["time"] = [{"text":"impl Sub&lt;Duration&gt; for Duration","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for Timespec","synthetic":false,"types":[]},{"text":"impl Sub&lt;Timespec&gt; for Timespec","synthetic":false,"types":[]},{"text":"impl Sub&lt;SteadyTime&gt; for SteadyTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for SteadyTime","synthetic":false,"types":[]},{"text":"impl Sub&lt;Duration&gt; for Tm","synthetic":false,"types":[]},{"text":"impl Sub&lt;Tm&gt; for Tm","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()