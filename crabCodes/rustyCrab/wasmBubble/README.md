<p>WASM > Generates random numbers and computes Bubble Sort.</p>
<ul>
  <li>Check val at Console.</li>
  <li>WASM Compilation by $ wasm-pack build --release --target web</li>
  <li>Host by $ python3 -m http.server --cgi 8080 </li>
  <li>Line 1 is, Random Numbers Generated</li>
  <li>Line 2 is, Items sorted by Bubble Sort</li>
</ul> 

<p> Probably there is a way to push items from console to webpage. 
    I am not sure how to do that at the moment, without the following fix.
I tried overriding the console.log func to fetch values which encounters recursion </p>
<p> Waiting for <a href="https://github.com/rustwasm/wasm-bindgen/issues/111">RUST WASM Fix</a></p>
