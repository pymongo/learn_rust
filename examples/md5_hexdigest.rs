/*
```python
import hashlib
hasher = hashlib.new('md5')
hasher.update("apple".encode())  # or b"apple"
assert isinstance(hasher.hexdigest(), str)
```

```ruby
require 'digest'
md5 = Digest::MD5.new
md5.update "apple"  # or md5 << "apple"
md5.hexdigest.is_a? String
```
*/
use md5::{Digest, Md5};

fn main() {
    let mut hasher = Md5::new();
    hasher.update("apple".as_bytes());
    dbg!(format!("{:x}", hasher.finalize()));
}
