#include <iostream>

// Perform one step of min-plus matrix multiplication
// n deontes the number of nodes
// d contains the input matrix ex: (i, j) = d[n*i + j]
// r contains the result
void step(float *r, const float *d, int n);

int main() {
  constexpr int n = 3;
  const float d[n * n] = {
      0, 8, 2, 1, 0, 9, 4, 5, 0,
  };
  float r[n * n];
  step(r, d, n);
  for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
      std::cout << r[i * n + j] << " ";
    }
    std::cout << "\n";
  }
}