# Latin Square

30x30 latin square solved using smallest-remaining-domain selection and upper-half reduction.

The `object` model encodes the `node` and `unit` into an `atom` object and it encodes this `atom` along with each `link` in a `fact` object. The `scalar` model encodes the `node` and `unit` into an `atom` scalar using a row-major format, and it encodes this `atom` along with each `link` into another `fact` scalar again using a row-major format. 

## Benchmark

Mean reported in milliseconds. Standard deviation reported in % of the mean. 

The fastest benchmark is with `scalar F5F Q1  C4F P3 ` at `61`. The fastest benchmark while using the `object` model is with `object F2X Q1  C1X P3 ` at `222` with non-secure hashing and `object F3R Q1  C1X P3 ` at `241` with secure hashing.

Statistics from [kern-bench.csv](/puzzles/latin/kern-bench.csv):

| name                     | mean  | stdv |
|--------------------------|-------|------|
| `scalar F5F Q1  C4F P3 ` | 61    | 2.0  |
| `scalar F5F Q1  C3  P3 ` | 63    | 3.7  |
| `scalar F5F Q4F C3  P3 ` | 63    | 1.2  |
| `scalar F5F Q4F C4F P3 ` | 63    | 0.4  |
| `scalar F5F Q1  C4M P3 ` | 64    | 2.8  |
| `scalar F5F Q4F C4M P3 ` | 67    | 0.9  |
| `scalar F5F Q4M C3  P3 ` | 74    | 2.5  |
| `scalar F5F Q1  C5F P3 ` | 75    | 1.7  |
| `scalar F5F Q4M C4F P3 ` | 76    | 0.4  |
| `scalar F2X Q1  C4F P3 ` | 76    | 1.1  |
| `scalar F3X Q1  C4F P3 ` | 77    | 2.4  |
| `scalar F5F Q4F C5F P3 ` | 78    | 0.2  |
| `scalar F2X Q1  C3  P3 ` | 79    | 0.4  |
| `scalar F2X Q1  C4M P3 ` | 79    | 0.3  |
| `scalar F3X Q1  C4M P3 ` | 80    | 1.0  |
| `scalar F5M Q1  C4F P3 ` | 81    | 0.8  |
| `scalar F5M Q1  C4M P3 ` | 81    | 0.5  |
| `scalar F3X Q1  C3  P3 ` | 81    | 3.5  |
| `scalar F5F Q3M C3  P3 ` | 82    | 1.0  |
| `scalar F5F Q4M C4M P3 ` | 82    | 0.5  |
| `scalar F5M Q1  C3  P3 ` | 83    | 1.5  |
| `scalar F5M Q4F C4F P3 ` | 83    | 0.5  |
| `scalar F3X Q4F C4F P3 ` | 83    | 1.4  |
| `scalar F5F Q3M C4F P3 ` | 84    | 0.9  |
| `scalar F5F Q1  C5M P3 ` | 84    | 1.9  |
| `scalar F2X Q4F C4F P3 ` | 84    | 0.6  |
| `scalar F3X Q4F C3  P3 ` | 85    | 1.9  |
| `scalar F2X Q4F C3  P3 ` | 85    | 0.3  |
| `scalar F5M Q4F C3  P3 ` | 85    | 1.8  |
| `scalar F5M Q4F C4M P3 ` | 86    | 0.5  |
| `scalar F5F Q4F C5M P3 ` | 86    | 0.1  |
| `scalar F3X Q4F C4M P3 ` | 87    | 1.9  |
| `scalar F2X Q4F C4M P3 ` | 87    | 0.3  |
| `scalar F5F Q3M C4M P3 ` | 89    | 9.3  |
| `scalar F1X Q1  C4F P3 ` | 90    | 0.3  |
| `scalar F2X Q4M C3  P3 ` | 91    | 1.0  |
| `scalar F5M Q1  C5F P3 ` | 92    | 0.2  |
| `scalar F2X Q1  C5F P3 ` | 92    | 0.3  |
| `scalar F3X Q1  C5F P3 ` | 92    | 0.7  |
| `scalar F5F Q4M C5F P3 ` | 92    | 0.1  |
| `scalar F5M Q4M C3  P3 ` | 93    | 0.6  |
| `scalar F1X Q1  C3  P3 ` | 93    | 1.8  |
| `scalar F2X Q4M C4F P3 ` | 93    | 0.5  |
| `scalar F1X Q1  C4M P3 ` | 93    | 0.6  |
| `scalar F3R Q1  C4F P3 ` | 94    | 4.5  |
| `scalar F1X Q4F C4F P3 ` | 94    | 1.2  |
| `scalar F5M Q4M C4F P3 ` | 94    | 0.5  |
| `scalar F3X Q4M C3  P3 ` | 95    | 1.9  |
| `scalar F3X Q4M C4F P3 ` | 96    | 1.8  |
| `scalar F5M Q4F C5F P3 ` | 96    | 0.3  |
| `scalar F1X Q4F C3  P3 ` | 97    | 1.6  |
| `scalar F2X Q4M C4M P3 ` | 97    | 1.2  |
| `scalar F2X Q4F C5F P3 ` | 97    | 0.3  |
| `scalar F5M Q4M C4M P3 ` | 97    | 0.4  |
| `scalar F5M Q1  C5M P3 ` | 98    | 0.2  |
| `scalar F3R Q1  C4M P3 ` | 99    | 7.6  |
| `scalar F3R Q1  C3  P3 ` | 99    | 7.1  |
| `scalar F1X Q4F C4M P3 ` | 99    | 1.0  |
| `scalar F3X Q4M C4M P3 ` | 100   | 1.5  |
| `scalar F3X Q4F C5F P3 ` | 100   | 1.5  |
| `scalar F5F Q3M C5F P3 ` | 100   | 0.3  |
| `scalar F2X Q1  C5M P3 ` | 100   | 0.2  |
| `scalar F3X Q1  C5M P3 ` | 101   | 0.7  |
| `scalar F5F Q4M C5M P3 ` | 101   | 0.2  |
| `scalar F5M Q3M C3  P3 ` | 102   | 0.2  |
| `scalar F3R Q4F C4F P3 ` | 102   | 11.1 |
| `scalar F5M Q4F C5M P3 ` | 103   | 0.2  |
| `scalar F5M Q3M C4F P3 ` | 103   | 0.8  |
| `scalar F1X Q1  C5F P3 ` | 104   | 0.6  |
| `scalar F3R Q4F C4M P3 ` | 104   | 6.9  |
| `scalar F2X Q3M C3  P3 ` | 105   | 2.0  |
| `scalar F3R Q4F C3  P3 ` | 105   | 11.9 |
| `scalar F5M Q3M C4M P3 ` | 105   | 0.6  |
| `scalar F2X Q4F C5M P3 ` | 105   | 0.5  |
| `scalar F1X Q4M C3  P3 ` | 106   | 1.3  |
| `scalar F2X Q3M C4F P3 ` | 107   | 0.3  |
| `scalar F5F Q3M C5M P3 ` | 108   | 0.2  |
| `scalar F1X Q4M C4F P3 ` | 108   | 1.6  |
| `scalar F5M Q4M C5F P3 ` | 108   | 0.3  |
| `scalar F3X Q4F C5M P3 ` | 108   | 2.2  |
| `scalar F3X Q3M C3  P3 ` | 109   | 2.2  |
| `scalar F2X Q3M C4M P3 ` | 109   | 0.8  |
| `scalar F2X Q4M C5F P3 ` | 110   | 0.6  |
| `scalar F3X Q3M C4F P3 ` | 110   | 2.4  |
| `scalar F1X Q1  C5M P3 ` | 111   | 1.3  |
| `scalar F1X Q4F C5F P3 ` | 112   | 2.1  |
| `scalar F1X Q4M C4M P3 ` | 112   | 1.1  |
| `scalar F3X Q3M C4M P3 ` | 112   | 2.4  |
| `scalar F3X Q4M C5F P3 ` | 112   | 1.4  |
| `scalar F3R Q4M C3  P3 ` | 113   | 6.2  |
| `scalar F5F Q4F C1X P3 ` | 113   | 0.8  |
| `scalar F3R Q1  C5F P3 ` | 114   | 9.2  |
| `scalar F5F Q4F C2X P3 ` | 114   | 2.2  |
| `scalar F5M Q4M C5M P3 ` | 116   | 0.2  |
| `scalar F3R Q4M C4F P3 ` | 116   | 13.2 |
| `scalar F1X Q3M C3  P3 ` | 117   | 1.6  |
| `scalar F3R Q4M C4M P3 ` | 118   | 4.6  |
| `scalar F3R Q4F C5F P3 ` | 118   | 7.3  |
| `scalar F1X Q3M C4F P3 ` | 118   | 1.1  |
| `scalar F5M Q3M C5F P3 ` | 118   | 0.2  |
| `scalar F2X Q4M C5M P3 ` | 119   | 0.2  |
| `scalar F1X Q4F C5M P3 ` | 120   | 0.9  |
| `scalar F3R Q1  C5M P3 ` | 121   | 5.7  |
| `scalar F1X Q3M C4M P3 ` | 121   | 2.1  |
| `scalar F3X Q4M C5M P3 ` | 122   | 2.0  |
| `scalar F5F Q1  C2X P3 ` | 123   | 3.1  |
| `scalar F2X Q3M C5F P3 ` | 123   | 0.2  |
| `scalar F5F Q1  C1X P3 ` | 124   | 5.4  |
| `scalar F1X Q4M C5F P3 ` | 125   | 1.3  |
| `scalar F5M Q3M C5M P3 ` | 126   | 0.2  |
| `scalar F3X Q3M C5F P3 ` | 126   | 1.5  |
| `scalar F5F Q4M C2X P3 ` | 127   | 0.7  |
| `scalar F5F Q4M C1X P3 ` | 128   | 1.4  |
| `scalar F3R Q4F C5M P3 ` | 128   | 10.4 |
| `scalar F2X Q1  C2X P3 ` | 129   | 1.1  |
| `scalar F2X Q1  C1X P3 ` | 129   | 1.0  |
| `scalar F5M Q1  C2X P3 ` | 131   | 0.4  |
| `scalar F3X Q1  C1X P3 ` | 131   | 2.0  |
| `scalar F3X Q1  C2X P3 ` | 131   | 1.9  |
| `scalar F2X Q3M C5M P3 ` | 132   | 0.2  |
| `scalar F5F Q2X C4F P3 ` | 133   | 7.1  |
| `scalar F5M Q1  C1X P3 ` | 134   | 3.2  |
| `scalar F1X Q4M C5M P3 ` | 134   | 1.0  |
| `scalar F3X Q2X C3  P3 ` | 134   | 3.1  |
| `scalar F2X Q2X C3  P3 ` | 134   | 3.4  |
| `scalar F2X Q2X C4F P3 ` | 134   | 3.4  |
| `scalar F3R Q3M C4F P3 ` | 135   | 6.9  |
| `scalar F4  Q3M C3  P3 ` | 135   | 1.7  |
| `scalar F5F Q3M C1X P3 ` | 135   | 0.8  |
| `scalar F5F Q3M C2X P3 ` | 135   | 1.1  |
| `scalar F3X Q3M C5M P3 ` | 135   | 1.5  |
| `scalar F3R Q3M C3  P3 ` | 135   | 12.7 |
| `scalar F3R Q3M C4M P3 ` | 136   | 2.5  |
| `scalar F2X Q4F C1X P3 ` | 136   | 0.8  |
| `scalar F1X Q3M C5F P3 ` | 136   | 0.9  |
| `scalar F2X Q4F C2X P3 ` | 137   | 0.6  |
| `scalar F3R Q4M C5F P3 ` | 137   | 14.5 |
| `scalar F5M Q4F C2X P3 ` | 137   | 1.1  |
| `scalar F5M Q4F C1X P3 ` | 137   | 0.4  |
| `scalar F5F Q2X C3  P3 ` | 138   | 4.6  |
| `scalar F3X Q2X C4F P3 ` | 138   | 2.7  |
| `scalar F4  Q3M C4F P3 ` | 138   | 1.9  |
| `scalar F2X Q2X C4M P3 ` | 140   | 1.5  |
| `scalar F5F Q2X C4M P3 ` | 143   | 7.4  |
| `scalar F3X Q2X C4M P3 ` | 143   | 2.8  |
| `scalar F1X Q3M C5M P3 ` | 144   | 1.0  |
| `scalar F4  Q3M C4M P3 ` | 145   | 1.2  |
| `scalar F3R Q4M C5M P3 ` | 145   | 12.3 |
| `scalar F5M Q2X C3  P3 ` | 145   | 1.3  |
| `scalar F5M Q2X C4F P3 ` | 146   | 0.8  |
| `scalar F1X Q2X C3  P3 ` | 146   | 3.9  |
| `scalar F1X Q1  C2X P3 ` | 147   | 1.4  |
| `scalar F1X Q1  C1X P3 ` | 147   | 1.6  |
| `scalar F5M Q4M C2X P3 ` | 148   | 1.3  |
| `scalar F5M Q2X C4M P3 ` | 149   | 0.3  |
| `scalar F5M Q4M C1X P3 ` | 149   | 2.0  |
| `scalar F1X Q2X C4F P3 ` | 150   | 5.4  |
| `scalar F1X Q2X C4M P3 ` | 150   | 2.8  |
| `scalar F2X Q4M C1X P3 ` | 151   | 0.4  |
| `scalar F2X Q4M C2X P3 ` | 151   | 1.2  |
| `scalar F3R Q1  C1X P3 ` | 153   | 6.1  |
| `scalar F3R Q3M C5F P3 ` | 154   | 5.8  |
| `scalar F3R Q1  C2X P3 ` | 155   | 9.9  |
| `scalar F3X Q4F C1X P3 ` | 157   | 2.0  |
| `scalar F3X Q4F C2X P3 ` | 157   | 3.2  |
| `scalar F5M Q3M C2X P3 ` | 158   | 1.0  |
| `scalar F5M Q3M C1X P3 ` | 158   | 0.8  |
| `scalar F2X Q3M C2X P3 ` | 160   | 0.6  |
| `scalar F4  Q3M C5F P3 ` | 161   | 2.2  |
| `scalar F2X Q3M C1X P3 ` | 161   | 0.4  |
| `scalar F3R Q3M C5M P3 ` | 161   | 3.5  |
| `scalar F3R Q4F C1X P3 ` | 163   | 7.8  |
| `scalar F3R Q4F C2X P3 ` | 165   | 12.2 |
| `scalar F4  Q1  C5F P3 ` | 165   | 0.8  |
| `scalar F5M Q2X C5F P3 ` | 166   | 0.3  |
| `scalar F2R Q4F C4F P3 ` | 167   | 4.5  |
| `scalar F3X Q2X C5F P3 ` | 168   | 0.9  |
| `scalar F2X Q2X C5F P3 ` | 168   | 1.4  |
| `scalar F2R Q1  C4F P3 ` | 168   | 8.3  |
| `scalar F3X Q4M C1X P3 ` | 170   | 2.3  |
| `scalar F3X Q4M C2X P3 ` | 170   | 2.4  |
| `scalar F2R Q1  C4M P3 ` | 171   | 5.3  |
| `scalar F2R Q1  C3  P3 ` | 171   | 6.2  |
| `scalar F2R Q4F C3  P3 ` | 172   | 10.0 |
| `scalar F2R Q4F C4M P3 ` | 173   | 9.2  |
| `scalar F5M Q2X C5M P3 ` | 175   | 1.2  |
| `scalar F4  Q3M C5M P3 ` | 175   | 2.1  |
| `scalar F3R Q4M C1X P3 ` | 176   | 6.7  |
| `scalar F3R Q4M C2X P3 ` | 176   | 9.4  |
| `scalar F1X Q4F C2X P3 ` | 178   | 2.4  |
| `scalar F2X Q2X C5M P3 ` | 178   | 3.6  |
| `scalar F2R Q4M C4F P3 ` | 179   | 3.0  |
| `scalar F3X Q2X C5M P3 ` | 179   | 1.3  |
| `scalar F1X Q4F C1X P3 ` | 180   | 2.4  |
| `scalar F1X Q2X C5F P3 ` | 180   | 2.2  |
| `scalar F2R Q4M C3  P3 ` | 181   | 7.3  |
| `scalar F2R Q4M C4M P3 ` | 183   | 2.4  |
| `scalar F3R Q2X C3  P3 ` | 183   | 12.5 |
| `scalar F3R Q2X C4M P3 ` | 183   | 7.3  |
| `scalar F2R Q1  C5F P3 ` | 184   | 3.5  |
| `scalar F3R Q2X C4F P3 ` | 184   | 6.6  |
| `scalar F2R Q4F C5F P3 ` | 185   | 7.5  |
| `scalar F5F Q4F C2R P3 ` | 188   | 0.5  |
| `scalar F4  Q1  C5M P3 ` | 188   | 0.8  |
| `scalar F1X Q2X C5M P3 ` | 190   | 1.8  |
| `scalar F2R Q4F C5M P3 ` | 193   | 4.7  |
| `scalar F1X Q4M C2X P3 ` | 193   | 2.6  |
| `scalar F2R Q1  C5M P3 ` | 194   | 3.1  |
| `scalar F1X Q4M C1X P3 ` | 195   | 1.9  |
| `scalar F5F Q2X C5F P3 ` | 195   | 10.8 |
| `scalar F3R Q3M C1X P3 ` | 196   | 5.1  |
| `scalar F3X Q3M C2X P3 ` | 197   | 6.9  |
| `scalar F2R Q3M C4F P3 ` | 197   | 4.4  |
| `scalar F3R Q3M C2X P3 ` | 199   | 10.3 |
| `scalar F2R Q3M C3  P3 ` | 199   | 11.0 |
| `scalar F1X Q3M C2X P3 ` | 199   | 2.0  |
| `scalar F1R Q1  C4F P3 ` | 199   | 0.8  |
| `scalar F2R Q4M C5F P3 ` | 199   | 8.7  |
| `scalar F3R Q2X C5F P3 ` | 200   | 7.3  |
| `scalar F1X Q3M C1X P3 ` | 201   | 2.0  |
| `scalar F1R Q1  C3  P3 ` | 203   | 1.3  |
| `scalar F1R Q1  C4M P3 ` | 203   | 1.1  |
| `scalar F1R Q4F C4F P3 ` | 203   | 1.2  |
| `scalar F5F Q4M C2R P3 ` | 203   | 0.6  |
| `scalar F5M Q1  C2R P3 ` | 204   | 0.8  |
| `scalar F1R Q4F C3  P3 ` | 205   | 1.1  |
| `scalar F2R Q3M C4M P3 ` | 206   | 14.4 |
| `scalar F1R Q4F C4M P3 ` | 206   | 0.8  |
| `scalar F3X Q3M C1X P3 ` | 207   | 3.5  |
| `scalar F3R Q2X C5M P3 ` | 208   | 4.2  |
| `scalar F5M Q4F C2R P3 ` | 209   | 0.4  |
| `scalar F5F Q4F C1R P3 ` | 209   | 0.3  |
| `scalar F2X Q1  C2R P3 ` | 210   | 0.7  |
| `scalar F2R Q4M C5M P3 ` | 211   | 10.6 |
| `scalar F1R Q4M C4F P3 ` | 214   | 0.5  |
| `scalar F5F Q3M C2R P3 ` | 214   | 0.5  |
| `scalar F1R Q4M C3  P3 ` | 214   | 0.9  |
| `scalar F3X Q1  C2R P3 ` | 214   | 1.4  |
| `scalar F5F Q2X C5M P3 ` | 215   | 10.0 |
| `scalar F2X Q4F C2R P3 ` | 216   | 0.7  |
| `scalar F1R Q4M C4M P3 ` | 218   | 0.8  |
| `scalar F1R Q4F C5F P3 ` | 219   | 0.6  |
| `scalar F1R Q1  C5F P3 ` | 220   | 2.1  |
| `scalar F2R Q3M C5F P3 ` | 221   | 11.8 |
| `scalar F5M Q2R C3  P3 ` | 221   | 0.4  |
| `scalar F5M Q2R C4F P3 ` | 221   | 0.4  |
| `scalar F5F Q1  C2R P3 ` | 222   | 5.9  |
| `scalar F5M Q2X C2X P3 ` | 222   | 0.8  |
| `scalar F5M Q2X C1X P3 ` | 222   | 0.9  |
| `object F2X Q1  C1X P3 ` | 222   | 1.8  |
| `scalar F1X Q1  C2R P3 ` | 223   | 1.5  |
| `scalar F5M Q1  C1R P3 ` | 225   | 0.2  |
| `scalar F5M Q2R C4M P3 ` | 225   | 0.6  |
| `scalar F2X Q2R C4F P3 ` | 225   | 0.5  |
| `scalar F5F Q4M C1R P3 ` | 225   | 0.9  |
| `scalar F5M Q4M C2R P3 ` | 225   | 0.7  |
| `scalar F1R Q1  C5M P3 ` | 226   | 1.1  |
| `scalar F3X Q2R C4F P3 ` | 227   | 2.3  |
| `scalar F2X Q2R C3  P3 ` | 227   | 0.6  |
| `scalar F1R Q3M C4F P3 ` | 227   | 0.7  |
| `scalar F1R Q3M C3  P3 ` | 228   | 1.4  |
| `scalar F2X Q2R C4M P3 ` | 228   | 0.6  |
| `scalar F2R Q3M C5M P3 ` | 228   | 10.1 |
| `scalar F1R Q4F C5M P3 ` | 229   | 0.6  |
| `scalar F3X Q2R C4M P3 ` | 229   | 1.2  |
| `scalar F2X Q2X C1X P3 ` | 230   | 1.0  |
| `scalar F2X Q2X C2X P3 ` | 231   | 1.0  |
| `object F3X Q1  C1X P3 ` | 231   | 4.2  |
| `scalar F1R Q3M C4M P3 ` | 231   | 0.6  |
| `scalar F5M Q4F C1R P3 ` | 231   | 0.5  |
| `scalar F3X Q2X C1X P3 ` | 232   | 2.7  |
| `scalar F5F Q2R C4M P3 ` | 232   | 2.4  |
| `scalar F2X Q1  C1R P3 ` | 232   | 0.5  |
| `scalar F2R Q4F C2X P3 ` | 232   | 5.6  |
| `scalar F1R Q4M C5F P3 ` | 233   | 1.1  |
| `scalar F5M Q3M C2R P3 ` | 233   | 0.6  |
| `scalar F3X Q2R C3  P3 ` | 233   | 4.0  |
| `scalar F3X Q2X C2X P3 ` | 234   | 4.1  |
| `scalar F5F Q2R C4F P3 ` | 234   | 3.3  |
| `scalar F3X Q1  C1R P3 ` | 234   | 1.3  |
| `scalar F4  Q3M C2X P3 ` | 235   | 3.0  |
| `scalar F4  Q3M C1X P3 ` | 235   | 2.8  |
| `scalar F2X Q4M C2R P3 ` | 235   | 0.6  |
| `scalar F1X Q2R C4F P3 ` | 236   | 1.3  |
| `scalar F5F Q3M C1R P3 ` | 237   | 0.4  |
| `scalar F2X Q4F C1R P3 ` | 237   | 1.2  |
| `scalar F1X Q2R C3  P3 ` | 238   | 0.9  |
| `scalar F2R Q4F C1X P3 ` | 238   | 7.1  |
| `scalar F1X Q2R C4M P3 ` | 241   | 0.9  |
| `scalar F5F Q1  C1R P3 ` | 241   | 6.5  |
| `object F3R Q1  C1X P3 ` | 241   | 10.7 |
| `scalar F1R Q4M C5M P3 ` | 242   | 1.4  |
| `scalar F1X Q1  C1R P3 ` | 243   | 1.0  |
| `scalar F5M Q2R C5F P3 ` | 243   | 0.3  |
| `scalar F2R Q4M C1X P3 ` | 243   | 4.4  |
| `scalar F1X Q2X C1X P3 ` | 243   | 2.2  |
| `scalar F1X Q2X C2X P3 ` | 244   | 2.2  |
| `scalar F2R Q4M C2X P3 ` | 244   | 9.8  |
| `scalar F2X Q3M C2R P3 ` | 245   | 0.4  |
| `scalar F1R Q3M C5F P3 ` | 245   | 0.7  |
| `scalar F5M Q4M C1R P3 ` | 247   | 0.3  |
| `scalar F2X Q2R C5F P3 ` | 247   | 0.5  |
| `object F1X Q1  C1X P3 ` | 248   | 2.8  |
| `scalar F5F Q2R C3  P3 ` | 249   | 13.6 |
| `scalar F5F Q2R C5F P3 ` | 249   | 4.5  |
| `scalar F3X Q2R C5F P3 ` | 249   | 0.7  |
| `scalar F3R Q1  C2R P3 ` | 250   | 16.3 |
| `scalar F3R Q4F C2R P3 ` | 252   | 9.4  |
| `scalar F1R Q3M C5M P3 ` | 253   | 0.9  |
| `scalar F1X Q2R C5F P3 ` | 254   | 0.8  |
| `scalar F5M Q2R C5M P3 ` | 254   | 0.2  |
| `scalar F2X Q2R C5M P3 ` | 255   | 0.8  |
| `scalar F3X Q2R C5M P3 ` | 256   | 0.8  |
| `scalar F2X Q4M C1R P3 ` | 258   | 0.5  |
| `scalar F5M Q3M C1R P3 ` | 258   | 0.4  |
| `scalar F2R Q1  C1X P3 ` | 259   | 4.6  |
| `scalar F5F Q2R C5M P3 ` | 259   | 3.3  |
| `scalar F3X Q4F C2R P3 ` | 260   | 2.1  |
| `scalar F3R Q2R C3  P3 ` | 262   | 4.9  |
| `scalar F1X Q2R C5M P3 ` | 263   | 0.9  |
| `scalar F2R Q3M C1X P3 ` | 263   | 10.2 |
| `scalar F2X Q3M C1R P3 ` | 263   | 0.4  |
| `scalar F2R Q3M C2X P3 ` | 263   | 8.8  |
| `scalar F3R Q2R C4M P3 ` | 264   | 9.0  |
| `scalar F3R Q2X C1X P3 ` | 265   | 5.5  |
| `scalar F3R Q1  C1R P3 ` | 265   | 9.4  |
| `scalar F3R Q2R C4F P3 ` | 266   | 11.5 |
| `scalar F3R Q2X C2X P3 ` | 266   | 5.7  |
| `scalar F1R Q4F C2X P3 ` | 267   | 1.0  |
| `scalar F2R Q1  C2X P3 ` | 267   | 12.8 |
| `scalar F1R Q4F C1X P3 ` | 268   | 1.3  |
| `scalar F1R Q1  C2X P3 ` | 271   | 3.0  |
| `scalar F3R Q4M C2R P3 ` | 271   | 9.0  |
| `scalar F3R Q4F C1R P3 ` | 274   | 6.5  |
| `scalar F1R Q1  C1X P3 ` | 275   | 3.1  |
| `scalar F1R Q4M C1X P3 ` | 277   | 0.9  |
| `scalar F1R Q4M C2X P3 ` | 277   | 1.0  |
| `scalar F3R Q2R C5F P3 ` | 279   | 6.2  |
| `scalar F3R Q3M C2R P3 ` | 280   | 2.5  |
| `scalar F3X Q4F C1R P3 ` | 286   | 2.1  |
| `scalar F1R Q2X C4F P3 ` | 287   | 2.4  |
| `scalar F1X Q4F C2R P3 ` | 287   | 2.1  |
| `scalar F3X Q4M C2R P3 ` | 288   | 2.4  |
| `scalar F1R Q2X C3  P3 ` | 289   | 2.1  |
| `scalar F1R Q2X C4M P3 ` | 292   | 2.0  |
| `scalar F1R Q3M C1X P3 ` | 293   | 1.0  |
| `scalar F1R Q3M C2X P3 ` | 293   | 0.9  |
| `scalar F3R Q2R C5M P3 ` | 296   | 13.5 |
| `scalar F3R Q4M C1R P3 ` | 296   | 11.1 |
| `scalar F5F Q2X C1X P3 ` | 300   | 15.7 |
| `scalar F5F Q2X C2X P3 ` | 301   | 16.2 |
| `scalar F1R Q2X C5F P3 ` | 304   | 1.4  |
| `scalar F3R Q3M C1R P3 ` | 308   | 6.9  |
| `object F2R Q1  C1X P3 ` | 311   | 8.5  |
| `scalar F5M Q2X C2R P3 ` | 311   | 0.8  |
| `scalar F3X Q4M C1R P3 ` | 314   | 2.6  |
| `scalar F1R Q2X C5M P3 ` | 314   | 1.3  |
| `scalar F5M Q2R C2X P3 ` | 318   | 0.7  |
| `scalar F1X Q3M C2R P3 ` | 319   | 3.6  |
| `scalar F5M Q2R C1X P3 ` | 319   | 1.6  |
| `scalar F2X Q2X C2R P3 ` | 319   | 1.0  |
| `scalar F1X Q4F C1R P3 ` | 321   | 2.7  |
| `scalar F2X Q2R C2X P3 ` | 321   | 0.9  |
| `scalar F3X Q2X C2R P3 ` | 322   | 2.7  |
| `scalar F2X Q2R C1X P3 ` | 323   | 0.9  |
| `scalar F1X Q4M C2R P3 ` | 324   | 2.1  |
| `scalar F2R Q4F C2R P3 ` | 328   | 14.2 |
| `scalar F2R Q4M C2R P3 ` | 334   | 3.3  |
| `scalar F5M Q2X C1R P3 ` | 335   | 0.6  |
| `scalar F1X Q2X C2R P3 ` | 336   | 1.4  |
| `scalar F3X Q2R C1X P3 ` | 336   | 7.3  |
| `scalar F1X Q2R C1X P3 ` | 337   | 1.8  |
| `scalar F1X Q3M C1R P3 ` | 337   | 1.7  |
| `scalar F1X Q2R C2X P3 ` | 337   | 1.8  |
| `scalar F3X Q3M C2R P3 ` | 340   | 2.4  |
| `scalar F2X Q2X C1R P3 ` | 341   | 1.0  |
| `scalar F3X Q2X C1R P3 ` | 344   | 1.3  |
| `scalar F1R Q4F C2R P3 ` | 346   | 1.0  |
| `scalar F2R Q3M C2R P3 ` | 348   | 6.5  |
| `scalar F1X Q4M C1R P3 ` | 350   | 1.8  |
| `scalar F2R Q4F C1R P3 ` | 350   | 13.9 |
| `scalar F1X Q2X C1R P3 ` | 355   | 1.5  |
| `scalar F1R Q1  C2R P3 ` | 356   | 4.2  |
| `object F2X Q1  C1R P3 ` | 356   | 1.9  |
| `scalar F2R Q4M C1R P3 ` | 358   | 3.6  |
| `scalar F1R Q4M C2R P3 ` | 362   | 1.4  |
| `scalar F3R Q2X C2R P3 ` | 366   | 6.5  |
| `scalar F3R Q2R C2X P3 ` | 366   | 7.3  |
| `scalar F1R Q4F C1R P3 ` | 368   | 1.0  |
| `object F3X Q1  C1R P3 ` | 368   | 8.8  |
| `scalar F3X Q3M C1R P3 ` | 369   | 2.5  |
| `scalar F1R Q2R C4F P3 ` | 372   | 7.7  |
| `scalar F1R Q2X C2X P3 ` | 372   | 1.7  |
| `scalar F3R Q2R C1X P3 ` | 372   | 11.8 |
| `scalar F1R Q2R C5F P3 ` | 373   | 1.4  |
| `scalar F1R Q2X C1X P3 ` | 373   | 2.1  |
| `scalar F1R Q3M C2R P3 ` | 374   | 1.0  |
| `scalar F2R Q3M C1R P3 ` | 377   | 8.1  |
| `object F1X Q1  C1R P3 ` | 380   | 2.7  |
| `scalar F1R Q1  C1R P3 ` | 380   | 3.7  |
| `scalar F4  Q3M C2R P3 ` | 380   | 3.0  |
| `scalar F1R Q2R C5M P3 ` | 381   | 0.8  |
| `scalar F1R Q2R C3  P3 ` | 383   | 8.1  |
| `scalar F1R Q4M C1R P3 ` | 384   | 0.9  |
| `object F3R Q1  C1R P3 ` | 384   | 9.5  |
| `scalar F3R Q2X C1R P3 ` | 388   | 4.0  |
| `scalar F5F Q2X C2R P3 ` | 390   | 6.5  |
| `scalar F1R Q2R C4M P3 ` | 391   | 9.6  |
| `scalar F2R Q1  C2R P3 ` | 392   | 8.6  |
| `scalar F1R Q3M C1R P3 ` | 394   | 0.9  |
| `object F1R Q1  C1X P3 ` | 399   | 1.0  |
| `scalar F2R Q2X C4F P3 ` | 401   | 7.4  |
| `scalar F5F Q2R C1X P3 ` | 402   | 7.0  |
| `scalar F5M Q2R C2R P3 ` | 403   | 0.6  |
| `scalar F2R Q2X C3  P3 ` | 405   | 10.2 |
| `scalar F2R Q2X C4M P3 ` | 408   | 3.7  |
| `scalar F5F Q2R C2X P3 ` | 415   | 13.3 |
| `scalar F4  Q3M C1R P3 ` | 415   | 3.2  |
| `scalar F2X Q2R C2R P3 ` | 417   | 1.1  |
| `scalar F2R Q2X C5F P3 ` | 417   | 9.4  |
| `scalar F3X Q2R C2X P3 ` | 419   | 12.9 |
| `scalar F2R Q1  C1R P3 ` | 422   | 8.3  |
| `scalar F2R Q2R C4M P3 ` | 424   | 7.4  |
| `scalar F3X Q2R C2R P3 ` | 424   | 5.5  |
| `scalar F5M Q2R C1R P3 ` | 426   | 0.9  |
| `scalar F1X Q2R C2R P3 ` | 431   | 1.7  |
| `scalar F2R Q2X C5M P3 ` | 433   | 11.4 |
| `scalar F2R Q2R C3  P3 ` | 440   | 6.0  |
| `scalar F2X Q2R C1R P3 ` | 440   | 1.3  |
| `scalar F2R Q2R C5F P3 ` | 442   | 7.5  |
| `scalar F5F Q2X C1R P3 ` | 454   | 11.2 |
| `scalar F1X Q2R C1R P3 ` | 455   | 1.6  |
| `scalar F2R Q2R C4F P3 ` | 455   | 10.3 |
| `scalar F2R Q2R C5M P3 ` | 461   | 10.3 |
| `scalar F1R Q2X C2R P3 ` | 461   | 1.9  |
| `object F2R Q1  C1R P3 ` | 464   | 4.7  |
| `scalar F3X Q2R C1R P3 ` | 469   | 11.5 |
| `scalar F3R Q2R C2R P3 ` | 479   | 12.1 |
| `scalar F1R Q2X C1R P3 ` | 484   | 1.9  |
| `scalar F1R Q2R C2X P3 ` | 493   | 4.6  |
| `scalar F3R Q2R C1R P3 ` | 502   | 12.4 |
| `scalar F5F Q2R C2R P3 ` | 515   | 6.1  |
| `object F1R Q1  C1R P3 ` | 538   | 1.5  |
| `scalar F2R Q2X C1X P3 ` | 550   | 10.3 |
| `scalar F1R Q2R C1X P3 ` | 562   | 11.9 |
| `scalar F2R Q2X C2X P3 ` | 582   | 6.8  |
| `scalar F2R Q2R C2X P3 ` | 644   | 7.0  |
| `scalar F5F Q2R C1R P3 ` | 654   | 9.5  |
| `scalar F1R Q2R C2R P3 ` | 656   | 12.0 |
| `scalar F1R Q2R C1R P3 ` | 675   | 8.1  |
| `scalar F2R Q2X C2R P3 ` | 713   | 8.3  |
| `scalar F2R Q2X C1R P3 ` | 720   | 3.5  |
| `scalar F4  Q1  C4F P3 ` | 726   | 0.4  |
| `scalar F2R Q2R C1X P3 ` | 726   | 12.1 |
| `scalar F4  Q1  C3  P3 ` | 760   | 0.5  |
| `scalar F4  Q1  C4M P3 ` | 775   | 0.4  |
| `scalar F2R Q2R C2R P3 ` | 890   | 10.0 |
| `scalar F2R Q2R C1R P3 ` | 975   | 9.9  |
| `scalar F4  Q4F C4F P3 ` | 1001  | 2.6  |
| `scalar F4  Q4F C3  P3 ` | 1007  | 0.8  |
| `scalar F4  Q4F C4M P3 ` | 1086  | 1.2  |
| `scalar F4  Q4F C5F P3 ` | 1125  | 5.3  |
| `scalar F4  Q4M C3  P3 ` | 1288  | 0.5  |
| `scalar F4  Q4M C4F P3 ` | 1318  | 0.7  |
| `scalar F4  Q4F C5M P3 ` | 1342  | 0.7  |
| `scalar F4  Q4M C4M P3 ` | 1371  | 0.5  |
| `scalar F4  Q4M C5F P3 ` | 1429  | 0.7  |
| `scalar F4  Q1  C1X P3 ` | 1460  | 2.0  |
| `scalar F4  Q1  C2X P3 ` | 1474  | 0.9  |
| `scalar F4  Q4M C5M P3 ` | 1620  | 0.5  |
| `object F4  Q1  C1X P3 ` | 1648  | 2.1  |
| `scalar F4  Q2X C3  P3 ` | 1834  | 1.3  |
| `scalar F4  Q2X C4F P3 ` | 1846  | 1.1  |
| `scalar F4  Q4F C1X P3 ` | 1900  | 1.7  |
| `scalar F4  Q4F C2X P3 ` | 1903  | 1.7  |
| `scalar F4  Q2X C4M P3 ` | 1912  | 1.1  |
| `scalar F4  Q2X C5F P3 ` | 1942  | 0.7  |
| `scalar F4  Q2X C5M P3 ` | 2156  | 0.6  |
| `scalar F4  Q4M C1X P3 ` | 2239  | 2.1  |
| `scalar F4  Q4M C2X P3 ` | 2329  | 1.7  |
| `scalar F4  Q1  C2R P3 ` | 2447  | 1.4  |
| `scalar F4  Q1  C1R P3 ` | 2825  | 4.3  |
| `scalar F4  Q2X C2X P3 ` | 2939  | 1.3  |
| `scalar F4  Q2X C1X P3 ` | 2954  | 3.1  |
| `scalar F4  Q4F C2R P3 ` | 3055  | 2.3  |
| `object F4  Q1  C1R P3 ` | 3241  | 3.6  |
| `scalar F4  Q4M C2R P3 ` | 3280  | 1.1  |
| `scalar F4  Q4F C1R P3 ` | 3619  | 5.2  |
| `scalar F4  Q2R C5F P3 ` | 3969  | 1.0  |
| `scalar F4  Q4M C1R P3 ` | 4005  | 6.3  |
| `scalar F4  Q2R C4M P3 ` | 4036  | 0.4  |
| `scalar F4  Q2R C4F P3 ` | 4043  | 2.1  |
| `scalar F4  Q2R C3  P3 ` | 4077  | 1.8  |
| `scalar F4  Q2X C2R P3 ` | 4124  | 0.5  |
| `scalar F4  Q2R C5M P3 ` | 4178  | 0.5  |
| `scalar F4  Q2X C1R P3 ` | 4659  | 3.0  |
| `scalar F4  Q2R C2X P3 ` | 5732  | 3.7  |
| `scalar F4  Q2R C1X P3 ` | 5990  | 10.7 |
| `scalar F4  Q2R C2R P3 ` | 6942  | 2.1  |
| `scalar F4  Q2R C1R P3 ` | 9595  | 2.3  |
| `scalar F5F Q3F C4M P3 ` | 11502 | 0.0  |
| `scalar F5M Q3F C4M P3 ` | 11505 | 0.0  |
| `scalar F5M Q3F C4F P3 ` | 11508 | 0.0  |
| `scalar F5M Q3F C3  P3 ` | 11512 | 0.0  |
| `scalar F5F Q3F C4F P3 ` | 11513 | 0.0  |
| `scalar F5M Q3F C5M P3 ` | 11521 | 0.0  |
| `scalar F5F Q3F C5F P3 ` | 11536 | 0.1  |
| `scalar F5F Q3F C5M P3 ` | 11537 | 0.1  |
| `scalar F5M Q3F C5F P3 ` | 11538 | 0.0  |
| `scalar F1X Q3F C4M P3 ` | 11548 | 0.1  |
| `scalar F1X Q3F C4F P3 ` | 11559 | 0.1  |
| `scalar F2X Q3F C4M P3 ` | 11560 | 0.1  |
| `scalar F2X Q3F C3  P3 ` | 11560 | 0.1  |
| `scalar F5M Q3F C2X P3 ` | 11577 | 0.0  |
| `scalar F5M Q3F C1X P3 ` | 11577 | 0.0  |
| `scalar F2X Q3F C5M P3 ` | 11580 | 0.0  |
| `scalar F2X Q3F C4F P3 ` | 11583 | 0.1  |
| `scalar F1X Q3F C5F P3 ` | 11585 | 0.1  |
| `scalar F5M Q3F C2R P3 ` | 11639 | 0.0  |
| `scalar F2X Q3F C5F P3 ` | 11647 | 0.4  |
| `scalar F5M Q3F C1R P3 ` | 11666 | 0.1  |
| `scalar F3X Q3F C3  P3 ` | 11667 | 0.5  |
| `scalar F3X Q3F C4M P3 ` | 11669 | 0.5  |
| `scalar F2X Q3F C2X P3 ` | 11681 | 0.1  |
| `scalar F2X Q3F C2R P3 ` | 11683 | 0.1  |
| `scalar F2X Q3F C1X P3 ` | 11689 | 0.1  |
| `scalar F1X Q3F C5M P3 ` | 11707 | 0.7  |
| `scalar F2X Q3F C1R P3 ` | 11713 | 0.1  |
| `scalar F5F Q3F C1R P3 ` | 11717 | 1.2  |
| `scalar F3X Q3F C1X P3 ` | 11725 | 0.5  |
| `scalar F1X Q3F C2R P3 ` | 11729 | 0.1  |
| `scalar F3X Q3F C4F P3 ` | 11734 | 0.6  |
| `scalar F3X Q3F C1R P3 ` | 11739 | 0.1  |
| `scalar F1R Q3F C4M P3 ` | 11763 | 0.2  |
| `scalar F1R Q3F C3  P3 ` | 11765 | 0.3  |
| `scalar F1R Q3F C5M P3 ` | 11774 | 0.2  |
| `scalar F3X Q3F C2R P3 ` | 11780 | 0.5  |
| `scalar F3X Q3F C5F P3 ` | 11802 | 0.6  |
| `scalar F5F Q3F C1X P3 ` | 11803 | 0.3  |
| `scalar F3X Q3F C5M P3 ` | 11815 | 0.5  |
| `scalar F3X Q3F C2X P3 ` | 11824 | 0.7  |
| `scalar F5F Q3F C2X P3 ` | 11826 | 0.3  |
| `scalar F1R Q3F C2R P3 ` | 11860 | 0.1  |
| `scalar F1R Q3F C4F P3 ` | 11890 | 0.5  |
| `scalar F1R Q3F C5F P3 ` | 11895 | 0.2  |
| `scalar F1X Q3F C1R P3 ` | 11916 | 0.6  |
| `scalar F1R Q3F C2X P3 ` | 11922 | 0.7  |
| `scalar F1X Q3F C3  P3 ` | 11959 | 0.3  |
| `scalar F1R Q3F C1X P3 ` | 11979 | 0.6  |
| `scalar F1R Q3F C1R P3 ` | 12135 | 0.8  |
| `scalar F5F Q3F C3  P3 ` | 12317 | 0.7  |
| `scalar F3R Q3F C3  P3 ` | 12724 | 8.1  |
| `scalar F3R Q3F C4M P3 ` | 12777 | 11.7 |
| `scalar F1X Q3F C1X P3 ` | 12885 | 0.6  |
| `scalar F1X Q3F C2X P3 ` | 12900 | 0.3  |
| `scalar F3R Q3F C1R P3 ` | 12911 | 6.8  |
| `scalar F3R Q3F C2R P3 ` | 12968 | 7.8  |
| `scalar F3R Q3F C5M P3 ` | 12968 | 10.7 |
| `scalar F2R Q3F C5M P3 ` | 13012 | 14.1 |
| `scalar F2R Q3F C4M P3 ` | 13024 | 12.7 |
| `scalar F5F Q3F C2R P3 ` | 13053 | 0.8  |
| `scalar F3R Q3F C4F P3 ` | 13432 | 6.0  |
| `scalar F3R Q3F C1X P3 ` | 13441 | 7.9  |
| `scalar F3R Q3F C2X P3 ` | 13467 | 8.5  |
| `scalar F3R Q3F C5F P3 ` | 13561 | 12.4 |
| `scalar F2R Q3F C3  P3 ` | 13828 | 10.5 |
| `scalar F2R Q3F C4F P3 ` | 13949 | 10.4 |
| `scalar F2R Q3F C2X P3 ` | 14033 | 9.6  |
| `scalar F2R Q3F C5F P3 ` | 14113 | 9.6  |
| `scalar F2R Q3F C1X P3 ` | 14173 | 10.8 |
| `scalar F2R Q3F C1R P3 ` | 14513 | 8.6  |
| `scalar F2R Q3F C2R P3 ` | 15195 | 11.1 |
| `scalar F4  Q3F C4F P3 ` | 19315 | 0.1  |
| `scalar F4  Q3F C4M P3 ` | 19393 | 0.3  |
| `scalar F4  Q3F C3  P3 ` | 19418 | 0.3  |
| `scalar F4  Q3F C5F P3 ` | 19461 | 0.3  |
| `scalar F4  Q3F C1R P3 ` | 19496 | 0.4  |
| `scalar F4  Q3F C5M P3 ` | 19519 | 0.4  |
| `scalar F4  Q3F C1X P3 ` | 19809 | 1.2  |
| `scalar F4  Q3F C2X P3 ` | 19892 | 0.4  |
| `scalar F4  Q3F C2R P3 ` | 20180 | 0.6  |

Reference through `minizinc` using [bench.py](/bench.py) on [model.mzn](/arc/puzzles/latin/zinc/model.mzn) with `n = 30`. Statistics from [zinc-bench.csv](/puzzles/latin/zinc-bench.csv):

| name     | mean | stdv |
|----------|------|------|
| `Gecode` | 163  | 5.3  |
| `CP-Sat` | nan  |      |
| `Gecode` | nan  |      |

## VTune UARCH Exploration

The kernel exhibited better instruction efficiency, lower back-end and memory pressure, better cache locality, and fewer serializing-operation stalls. It performs more useful CPU work than memory waiting.

### Kernel

```
Elapsed Time:                          1.305s
Clockticks:                    6,278,718,000
Instructions Retired:         23,311,968,000
CPI Rate:                              0.269
MUX Reliability:                       0.936

Retiring:                              68.2%

Front-End Bound:                       17.6%
  Front-End Latency:                   10.5%
  Front-End Bandwidth:                  7.0%

Bad Speculation:                        0.5%
  Branch Mispredict:                    3.8%

Back-End Bound:                        13.7%
  Memory Bound:                         6.9%
    L1 Bound:                           0.4%
    L2 Bound:                           0.0%
    L3 Bound:                           1.7%
    DRAM Bound:                         7.9%
    Store Bound:                        0.4%
  Core Bound:                           6.9%
    Divider:                            0.0%
    Serializing Operations:             3.4%
    Port Utilization:                  18.8%
```

### Gecode

```
Elapsed Time:                         11.268s
Clockticks:                   56,354,726,000
Instructions Retired:        123,995,072,000
CPI Rate:                              0.454
MUX Reliability:                       0.919

Retiring:                              34.7%

Front-End Bound:                       23.3%
  Front-End Latency:                    8.5%
  Front-End Bandwidth:                 14.8%

Bad Speculation:                        2.9%
  Branch Mispredict:                    2.2%

Back-End Bound:                        39.1%
  Memory Bound:                        22.8%
    L1 Bound:                          13.0%
    L2 Bound:                           2.3%
    L3 Bound:                           2.4%
    DRAM Bound:                         7.6%
    Store Bound:                        4.6%
  Core Bound:                          16.2%
    Divider:                            0.1%
    Serializing Operations:            11.5%
    Port Utilization:                  19.5%
```
