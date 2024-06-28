int min(vector<int>& nums) {
  int op = 0;
  for (int num:nums) {
    int esc = num % 3;
    int dir = ((num / 3) + 1) * 3 - nums;
    op += min(esq, dir);
  }
  return op;
}
