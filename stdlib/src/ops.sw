library ops;

trait EqNeq {
  fn eq(self, other: Self) -> bool;
} {
  fn neq(self, other: Self) -> bool {
    if self.eq(other) { false } else { true }
  }
}
impl EqNeq for u64 {
  fn eq(self, other: Self) -> bool {
     asm(r1: self, r2: other, r3) {
        eq r3 r1 r2;
        r3: bool
    }
  }

}