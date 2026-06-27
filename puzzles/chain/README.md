# chain Square

30x30 chain square solved using smallest-remaining-domain selection and upper-half reduction.

The `object` model encodes the `node` and `unit` into an `atom` object and it encodes this `atom` along with each `link` in a `fact` object. The `scalar` model encodes the `node` and `unit` into an `atom` scalar using a row-major format, and it encodes this `atom` along with each `link` into another `fact` scalar again using a row-major format. 

## Benchmark

Mean reported in milliseconds. Standard deviation reported in % of the mean. 

The fastest benchmark is with `scalar F5F Q3M C3  P3 ` at `162`. The fastest benchmark while using the `object` model is with `object F3X Q1  C1X P3 ` at `220` with non-secure hashing and `object F3R Q1  C1R P3 ` at `321` with secure hashing.

Statistics from [kern-bench.csv](/puzzles/chain/kern-bench.csv):

| name                     | mean  | stdv |
|--------------------------|-------|------|
| `scalar F5F Q3M C3  P3 ` | 162   | 1.2  |
| `scalar F3X Q1  C4F P3 ` | 164   | 1.1  |
| `scalar F5F Q3M C4F P3 ` | 164   | 1.1  |
| `scalar F3X Q1  C3  P3 ` | 164   | 1.1  |
| `scalar F5F Q3M C4M P3 ` | 165   | 1.9  |
| `scalar F2X Q4F C4F P3 ` | 166   | 1.1  |
| `scalar F2X Q4F C3  P3 ` | 166   | 1.3  |
| `scalar F3X Q4F C3  P3 ` | 167   | 1.4  |
| `scalar F3X Q4F C4F P3 ` | 167   | 1.5  |
| `scalar F3X Q1  C4M P3 ` | 168   | 2.6  |
| `scalar F2X Q4F C4M P3 ` | 168   | 1.5  |
| `scalar F2X Q1  C3  P3 ` | 168   | 1.9  |
| `scalar F3X Q4F C4M P3 ` | 168   | 1.3  |
| `scalar F2X Q1  C4F P3 ` | 168   | 1.9  |
| `scalar F2X Q1  C4M P3 ` | 169   | 1.4  |
| `scalar F2X Q4M C3  P3 ` | 169   | 1.3  |
| `scalar F2X Q4M C4F P3 ` | 170   | 1.4  |
| `scalar F3X Q4M C3  P3 ` | 171   | 1.8  |
| `scalar F4  Q3M C4F P3 ` | 171   | 2.1  |
| `scalar F3X Q4M C4F P3 ` | 171   | 1.9  |
| `scalar F4  Q3M C3  P3 ` | 172   | 2.0  |
| `scalar F2X Q4M C4M P3 ` | 172   | 2.3  |
| `scalar F3X Q4M C4M P3 ` | 173   | 1.6  |
| `scalar F4  Q3M C4M P3 ` | 177   | 5.0  |
| `scalar F5F Q3M C2R P3 ` | 180   | 1.2  |
| `scalar F5F Q3M C1X P3 ` | 188   | 1.0  |
| `scalar F5F Q3M C2X P3 ` | 188   | 1.0  |
| `scalar F5F Q3M C5F P3 ` | 192   | 1.0  |
| `scalar F3X Q1  C1X P3 ` | 194   | 1.4  |
| `scalar F3X Q1  C2X P3 ` | 195   | 0.9  |
| `scalar F3X Q1  C5F P3 ` | 197   | 2.1  |
| `scalar F3X Q4F C2X P3 ` | 197   | 1.2  |
| `scalar F2X Q4F C1X P3 ` | 198   | 1.3  |
| `scalar F3X Q4F C5F P3 ` | 198   | 1.4  |
| `scalar F2X Q4F C2X P3 ` | 198   | 1.9  |
| `scalar F2X Q1  C1X P3 ` | 199   | 1.6  |
| `scalar F4  Q3M C2R P3 ` | 199   | 2.6  |
| `scalar F3X Q4F C1X P3 ` | 199   | 1.5  |
| `scalar F2X Q4F C5F P3 ` | 199   | 1.6  |
| `scalar F2X Q4M C5F P3 ` | 200   | 1.4  |
| `scalar F2X Q1  C2X P3 ` | 200   | 2.3  |
| `scalar F4  Q3M C5F P3 ` | 201   | 2.0  |
| `scalar F3X Q2X C3  P3 ` | 201   | 4.9  |
| `scalar F2X Q4M C2X P3 ` | 201   | 1.4  |
| `scalar F5F Q3M C5M P3 ` | 201   | 0.7  |
| `scalar F3X Q4M C1X P3 ` | 202   | 1.7  |
| `scalar F3X Q4M C5F P3 ` | 202   | 2.0  |
| `scalar F3X Q4M C2X P3 ` | 202   | 1.6  |
| `scalar F5F Q3M C1R P3 ` | 202   | 1.1  |
| `scalar F3X Q2X C4M P3 ` | 202   | 2.3  |
| `scalar F2X Q4M C1X P3 ` | 203   | 1.7  |
| `scalar F2X Q1  C5F P3 ` | 205   | 1.9  |
| `scalar F3X Q1  C5M P3 ` | 205   | 1.3  |
| `scalar F2X Q2X C3  P3 ` | 207   | 4.2  |
| `scalar F2X Q4F C5M P3 ` | 209   | 1.2  |
| `scalar F4  Q3M C1X P3 ` | 210   | 5.3  |
| `scalar F3X Q4F C5M P3 ` | 210   | 1.5  |
| `scalar F3X Q2X C4F P3 ` | 211   | 5.9  |
| `scalar F4  Q3M C5M P3 ` | 211   | 2.0  |
| `scalar F2X Q4M C5M P3 ` | 211   | 1.0  |
| `scalar F3X Q4M C5M P3 ` | 212   | 1.1  |
| `scalar F4  Q3M C2X P3 ` | 212   | 5.7  |
| `scalar F2X Q2X C4F P3 ` | 213   | 7.0  |
| `scalar F2X Q2X C4M P3 ` | 215   | 4.1  |
| `scalar F2X Q1  C5M P3 ` | 215   | 1.3  |
| `scalar F2X Q4F C2R P3 ` | 218   | 1.4  |
| `scalar F3X Q4F C2R P3 ` | 218   | 1.3  |
| `scalar F3X Q1  C2R P3 ` | 218   | 1.4  |
| `object F3X Q1  C1X P3 ` | 220   | 0.9  |
| `object F2X Q1  C1X P3 ` | 220   | 0.9  |
| `scalar F2X Q4M C2R P3 ` | 221   | 1.0  |
| `scalar F3X Q4M C2R P3 ` | 224   | 1.7  |
| `scalar F3R Q1  C4F P3 ` | 227   | 0.8  |
| `scalar F2X Q1  C2R P3 ` | 227   | 3.0  |
| `scalar F3R Q1  C3  P3 ` | 227   | 0.9  |
| `scalar F3R Q1  C4M P3 ` | 227   | 0.7  |
| `scalar F3X Q2X C1X P3 ` | 231   | 1.7  |
| `scalar F4  Q3M C1R P3 ` | 232   | 6.5  |
| `scalar F3X Q2X C5F P3 ` | 234   | 3.2  |
| `scalar F3X Q2R C3  P3 ` | 234   | 1.7  |
| `scalar F3R Q4F C3  P3 ` | 235   | 1.2  |
| `scalar F3R Q4F C4F P3 ` | 235   | 0.9  |
| `scalar F3X Q2X C2X P3 ` | 237   | 4.7  |
| `scalar F3R Q4F C4M P3 ` | 237   | 1.2  |
| `scalar F3X Q2R C4M P3 ` | 238   | 2.5  |
| `scalar F3X Q1  C1R P3 ` | 238   | 1.4  |
| `scalar F2X Q2R C4F P3 ` | 239   | 3.1  |
| `scalar F3R Q4M C3  P3 ` | 241   | 1.1  |
| `scalar F2X Q4F C1R P3 ` | 241   | 1.1  |
| `scalar F3R Q4M C4F P3 ` | 242   | 1.6  |
| `scalar F3X Q4F C1R P3 ` | 242   | 1.4  |
| `scalar F2X Q2X C2X P3 ` | 243   | 4.3  |
| `scalar F3R Q4M C4M P3 ` | 243   | 1.2  |
| `scalar F2X Q2X C5F P3 ` | 243   | 4.3  |
| `scalar F2X Q1  C1R P3 ` | 243   | 1.8  |
| `scalar F3X Q2R C4F P3 ` | 244   | 6.2  |
| `scalar F3X Q4M C1R P3 ` | 245   | 1.2  |
| `scalar F3X Q2X C5M P3 ` | 246   | 1.6  |
| `scalar F2X Q2X C1X P3 ` | 247   | 7.2  |
| `scalar F2X Q4M C1R P3 ` | 247   | 1.9  |
| `scalar F2X Q2R C3  P3 ` | 247   | 5.1  |
| `scalar F2X Q2R C4M P3 ` | 249   | 4.2  |
| `scalar F3R Q1  C5F P3 ` | 249   | 0.8  |
| `scalar F3R Q1  C2X P3 ` | 256   | 0.9  |
| `scalar F2X Q2X C2R P3 ` | 256   | 2.8  |
| `scalar F3R Q1  C1X P3 ` | 256   | 0.8  |
| `scalar F3R Q4F C5F P3 ` | 259   | 1.2  |
| `scalar F3R Q4M C5F P3 ` | 262   | 2.2  |
| `scalar F3R Q1  C5M P3 ` | 262   | 0.6  |
| `scalar F3R Q4F C1X P3 ` | 263   | 1.1  |
| `scalar F3R Q4F C2X P3 ` | 263   | 1.2  |
| `scalar F3R Q2X C3  P3 ` | 264   | 2.6  |
| `scalar F3X Q2R C5F P3 ` | 264   | 1.6  |
| `scalar F3R Q2X C4F P3 ` | 264   | 2.3  |
| `scalar F2X Q2X C5M P3 ` | 265   | 4.6  |
| `scalar F2X Q3M C3  P3 ` | 265   | 0.8  |
| `scalar F2X Q3M C4M P3 ` | 265   | 0.8  |
| `scalar F3R Q2X C4M P3 ` | 266   | 2.4  |
| `scalar F3X Q2X C2R P3 ` | 266   | 7.5  |
| `scalar F3X Q3M C3  P3 ` | 266   | 0.9  |
| `scalar F2R Q1  C4M P3 ` | 267   | 0.9  |
| `scalar F2R Q1  C4F P3 ` | 267   | 1.2  |
| `scalar F3R Q4M C1X P3 ` | 267   | 1.0  |
| `scalar F3X Q3M C4M P3 ` | 267   | 1.0  |
| `scalar F2R Q4F C3  P3 ` | 268   | 0.9  |
| `scalar F2R Q4F C4F P3 ` | 268   | 1.3  |
| `scalar F3X Q2R C2X P3 ` | 268   | 1.3  |
| `scalar F3X Q2R C1X P3 ` | 268   | 1.4  |
| `scalar F2X Q3M C4F P3 ` | 268   | 0.9  |
| `scalar F3R Q3M C3  P3 ` | 269   | 1.1  |
| `scalar F2R Q1  C3  P3 ` | 269   | 1.1  |
| `scalar F3R Q4M C5M P3 ` | 269   | 1.0  |
| `scalar F3R Q4M C2X P3 ` | 269   | 1.2  |
| `scalar F2R Q4F C4M P3 ` | 270   | 1.1  |
| `scalar F3R Q3M C4F P3 ` | 270   | 1.2  |
| `scalar F3X Q3M C4F P3 ` | 271   | 0.9  |
| `object F3X Q1  C1R P3 ` | 271   | 0.6  |
| `object F2X Q1  C1R P3 ` | 271   | 0.7  |
| `scalar F3R Q1  C2R P3 ` | 271   | 0.7  |
| `scalar F3R Q3M C4M P3 ` | 273   | 1.0  |
| `scalar F3R Q4F C5M P3 ` | 274   | 0.8  |
| `object F3R Q1  C1X P3 ` | 275   | 0.7  |
| `scalar F2X Q2R C5F P3 ` | 275   | 2.3  |
| `scalar F3X Q2R C5M P3 ` | 276   | 1.2  |
| `scalar F3X Q2X C1R P3 ` | 277   | 1.4  |
| `scalar F2R Q4M C4F P3 ` | 277   | 1.1  |
| `scalar F2R Q4M C4M P3 ` | 279   | 1.1  |
| `scalar F2R Q4M C3  P3 ` | 280   | 1.5  |
| `scalar F3R Q2X C5F P3 ` | 281   | 1.6  |
| `scalar F2X Q2R C1X P3 ` | 285   | 4.3  |
| `scalar F3R Q4F C2R P3 ` | 285   | 1.1  |
| `scalar F3R Q2X C1X P3 ` | 285   | 1.2  |
| `scalar F2X Q2R C2X P3 ` | 285   | 5.0  |
| `scalar F3R Q2R C3  P3 ` | 287   | 0.9  |
| `scalar F2X Q2X C1R P3 ` | 289   | 5.0  |
| `scalar F3R Q2R C4M P3 ` | 289   | 1.5  |
| `scalar F3R Q2R C4F P3 ` | 289   | 1.0  |
| `scalar F2R Q1  C5F P3 ` | 289   | 0.9  |
| `scalar F2X Q3M C5F P3 ` | 289   | 0.9  |
| `scalar F3R Q4M C2R P3 ` | 289   | 1.6  |
| `scalar F5M Q3M C3  P3 ` | 290   | 1.1  |
| `scalar F5M Q3M C4M P3 ` | 290   | 1.1  |
| `scalar F3R Q1  C1R P3 ` | 291   | 0.7  |
| `scalar F2X Q3M C2X P3 ` | 291   | 0.7  |
| `scalar F2X Q3M C1X P3 ` | 291   | 1.0  |
| `scalar F3R Q2X C2X P3 ` | 291   | 2.1  |
| `scalar F5M Q3M C4F P3 ` | 291   | 1.1  |
| `scalar F3X Q2R C2R P3 ` | 291   | 2.0  |
| `scalar F2R Q4F C5F P3 ` | 292   | 1.2  |
| `scalar F3X Q3M C1X P3 ` | 292   | 1.0  |
| `scalar F3X Q3M C5F P3 ` | 292   | 1.5  |
| `scalar F2X Q2R C5M P3 ` | 293   | 2.7  |
| `scalar F3X Q3M C2X P3 ` | 293   | 1.0  |
| `scalar F3R Q3M C2X P3 ` | 296   | 1.2  |
| `scalar F3R Q2X C5M P3 ` | 296   | 1.5  |
| `scalar F3R Q3M C1X P3 ` | 297   | 1.2  |
| `scalar F2R Q4F C1X P3 ` | 298   | 1.1  |
| `scalar F2R Q1  C2X P3 ` | 298   | 1.2  |
| `scalar F2R Q4F C2X P3 ` | 298   | 0.9  |
| `scalar F2R Q4M C5F P3 ` | 301   | 1.1  |
| `scalar F3R Q4F C1R P3 ` | 302   | 1.1  |
| `scalar F2R Q1  C1X P3 ` | 302   | 2.0  |
| `scalar F5M Q3M C2R P3 ` | 304   | 1.2  |
| `scalar F2R Q3M C3  P3 ` | 305   | 1.1  |
| `scalar F3R Q4M C1R P3 ` | 306   | 1.0  |
| `scalar F2X Q3M C5M P3 ` | 306   | 0.6  |
| `scalar F3R Q2X C2R P3 ` | 307   | 1.0  |
| `scalar F3R Q3M C5F P3 ` | 308   | 1.0  |
| `scalar F2R Q3M C4M P3 ` | 308   | 0.8  |
| `scalar F2R Q4M C2X P3 ` | 308   | 1.7  |
| `scalar F2R Q1  C5M P3 ` | 309   | 2.2  |
| `scalar F3X Q3M C5M P3 ` | 309   | 1.5  |
| `scalar F2R Q4F C5M P3 ` | 309   | 2.0  |
| `scalar F2R Q4M C1X P3 ` | 310   | 2.2  |
| `scalar F2R Q3M C4F P3 ` | 311   | 1.0  |
| `scalar F3R Q2R C5F P3 ` | 311   | 0.8  |
| `scalar F5M Q3M C1X P3 ` | 311   | 1.6  |
| `scalar F2X Q2R C2R P3 ` | 311   | 5.8  |
| `object F2R Q1  C1X P3 ` | 313   | 0.4  |
| `scalar F2X Q3M C2R P3 ` | 313   | 0.8  |
| `scalar F3X Q2R C1R P3 ` | 314   | 0.9  |
| `scalar F3R Q3M C2R P3 ` | 315   | 1.0  |
| `scalar F3X Q3M C2R P3 ` | 315   | 0.9  |
| `scalar F5M Q3M C5F P3 ` | 315   | 1.0  |
| `scalar F2R Q2X C4M P3 ` | 317   | 2.3  |
| `scalar F2R Q4F C2R P3 ` | 318   | 0.9  |
| `scalar F2R Q1  C2R P3 ` | 318   | 1.8  |
| `scalar F3R Q2R C2X P3 ` | 318   | 0.8  |
| `scalar F3R Q2R C1X P3 ` | 319   | 1.0  |
| `scalar F2R Q4M C5M P3 ` | 320   | 1.7  |
| `scalar F2R Q2X C4F P3 ` | 321   | 2.0  |
| `object F3R Q1  C1R P3 ` | 321   | 0.6  |
| `scalar F2R Q2X C3  P3 ` | 322   | 2.1  |
| `scalar F3R Q2X C1R P3 ` | 324   | 1.0  |
| `scalar F3R Q3M C5M P3 ` | 325   | 0.9  |
| `scalar F3R Q2R C5M P3 ` | 325   | 0.8  |
| `scalar F5M Q3M C5M P3 ` | 327   | 0.9  |
| `scalar F5M Q3M C1R P3 ` | 327   | 1.6  |
| `scalar F2R Q4M C2R P3 ` | 329   | 1.2  |
| `scalar F5M Q3M C2X P3 ` | 330   | 4.9  |
| `scalar F2X Q2R C1R P3 ` | 331   | 4.6  |
| `scalar F2R Q2R C4F P3 ` | 335   | 1.8  |
| `scalar F2X Q3M C1R P3 ` | 336   | 1.0  |
| `scalar F3R Q3M C1R P3 ` | 336   | 1.8  |
| `scalar F2R Q2X C1X P3 ` | 336   | 1.8  |
| `scalar F3X Q3M C1R P3 ` | 336   | 1.0  |
| `scalar F2R Q2R C3  P3 ` | 337   | 1.6  |
| `scalar F2R Q4F C1R P3 ` | 337   | 1.1  |
| `scalar F2R Q2R C4M P3 ` | 337   | 2.0  |
| `scalar F2R Q3M C2X P3 ` | 337   | 1.2  |
| `scalar F2R Q3M C1X P3 ` | 338   | 1.1  |
| `scalar F2R Q3M C5F P3 ` | 338   | 0.9  |
| `scalar F2R Q1  C1R P3 ` | 339   | 1.9  |
| `scalar F3R Q2R C2R P3 ` | 340   | 1.4  |
| `scalar F2R Q2X C5F P3 ` | 342   | 2.7  |
| `scalar F2R Q4M C1R P3 ` | 344   | 0.9  |
| `scalar F2R Q3M C2R P3 ` | 352   | 1.8  |
| `scalar F2R Q2X C2X P3 ` | 356   | 2.4  |
| `scalar F2R Q3M C5M P3 ` | 357   | 1.2  |
| `scalar F3R Q2R C1R P3 ` | 357   | 0.8  |
| `scalar F2R Q2R C5F P3 ` | 359   | 3.1  |
| `object F2R Q1  C1R P3 ` | 363   | 0.7  |
| `scalar F2R Q2X C5M P3 ` | 365   | 2.5  |
| `scalar F2R Q2R C2X P3 ` | 367   | 1.8  |
| `scalar F2R Q3M C1R P3 ` | 370   | 0.9  |
| `scalar F2R Q2R C5M P3 ` | 376   | 3.0  |
| `scalar F2R Q2R C1X P3 ` | 378   | 3.3  |
| `scalar F2R Q2X C2R P3 ` | 391   | 8.4  |
| `scalar F2R Q2X C1R P3 ` | 392   | 4.2  |
| `scalar F2R Q2R C2R P3 ` | 405   | 5.1  |
| `scalar F2R Q2R C1R P3 ` | 432   | 3.4  |
| `scalar F5F Q1  C3  P3 ` | 721   | 1.3  |
| `scalar F5F Q1  C5F P3 ` | 759   | 0.7  |
| `scalar F5F Q1  C4F P3 ` | 760   | 1.7  |
| `scalar F5F Q4F C3  P3 ` | 764   | 1.0  |
| `scalar F5F Q4F C4F P3 ` | 764   | 1.2  |
| `scalar F5F Q1  C4M P3 ` | 768   | 1.5  |
| `scalar F5F Q4M C3  P3 ` | 792   | 1.3  |
| `scalar F5F Q4F C4M P3 ` | 794   | 1.1  |
| `scalar F5F Q4F C5F P3 ` | 796   | 0.4  |
| `scalar F5F Q4M C4F P3 ` | 824   | 1.1  |
| `scalar F5F Q4M C5F P3 ` | 833   | 0.6  |
| `scalar F5F Q4M C4M P3 ` | 862   | 1.1  |
| `scalar F5F Q1  C5M P3 ` | 869   | 0.5  |
| `scalar F1X Q3M C4F P3 ` | 903   | 0.2  |
| `scalar F5F Q4F C5M P3 ` | 904   | 0.4  |
| `scalar F1X Q3M C3  P3 ` | 904   | 0.2  |
| `scalar F1X Q3M C4M P3 ` | 904   | 0.2  |
| `scalar F1X Q3M C2R P3 ` | 925   | 0.3  |
| `scalar F1X Q3M C2X P3 ` | 933   | 0.2  |
| `scalar F1X Q3M C5F P3 ` | 933   | 0.1  |
| `scalar F1X Q3M C1X P3 ` | 933   | 0.2  |
| `scalar F1X Q3M C5M P3 ` | 944   | 0.2  |
| `scalar F5F Q4M C5M P3 ` | 951   | 0.2  |
| `scalar F1X Q3M C1R P3 ` | 956   | 0.2  |
| `scalar F5F Q1  C2X P3 ` | 1020  | 0.7  |
| `scalar F5F Q1  C1X P3 ` | 1022  | 1.6  |
| `scalar F5F Q4F C1X P3 ` | 1023  | 0.4  |
| `scalar F5F Q4F C2X P3 ` | 1023  | 0.4  |
| `scalar F5M Q1  C5F P3 ` | 1026  | 0.3  |
| `scalar F5M Q1  C3  P3 ` | 1033  | 0.7  |
| `scalar F5M Q1  C4F P3 ` | 1040  | 1.1  |
| `scalar F5M Q1  C4M P3 ` | 1066  | 0.7  |
| `scalar F5M Q4F C3  P3 ` | 1067  | 0.7  |
| `scalar F5F Q4M C2X P3 ` | 1072  | 0.6  |
| `scalar F5M Q4F C4F P3 ` | 1072  | 1.0  |
| `scalar F5M Q4F C5F P3 ` | 1083  | 0.4  |
| `scalar F5F Q3F C4M P3 ` | 1087  | 0.3  |
| `scalar F5M Q4F C4M P3 ` | 1103  | 0.9  |
| `scalar F5F Q3F C2R P3 ` | 1107  | 0.5  |
| `scalar F5F Q3F C5F P3 ` | 1116  | 0.5  |
| `scalar F5M Q4M C5F P3 ` | 1123  | 0.5  |
| `scalar F5F Q3F C5M P3 ` | 1123  | 0.4  |
| `scalar F5F Q3F C1X P3 ` | 1125  | 0.5  |
| `scalar F5M Q4M C3  P3 ` | 1125  | 1.2  |
| `scalar F5F Q3F C2X P3 ` | 1127  | 0.6  |
| `scalar F4  Q3F C3  P3 ` | 1128  | 0.4  |
| `scalar F5M Q1  C5M P3 ` | 1131  | 0.2  |
| `scalar F5M Q4M C4F P3 ` | 1143  | 1.2  |
| `scalar F4  Q3F C4F P3 ` | 1143  | 1.1  |
| `scalar F4  Q3F C4M P3 ` | 1147  | 0.7  |
| `scalar F4  Q3F C1X P3 ` | 1156  | 0.3  |
| `scalar F4  Q3F C2X P3 ` | 1156  | 0.3  |
| `scalar F4  Q3F C5M P3 ` | 1162  | 0.2  |
| `scalar F4  Q3F C2R P3 ` | 1166  | 0.5  |
| `scalar F5F Q2X C5F P3 ` | 1167  | 2.8  |
| `scalar F5M Q4F C5M P3 ` | 1170  | 0.3  |
| `scalar F4  Q3F C1R P3 ` | 1173  | 0.7  |
| `scalar F5F Q3F C4F P3 ` | 1175  | 2.0  |
| `scalar F4  Q3F C5F P3 ` | 1176  | 0.5  |
| `scalar F5M Q4M C4M P3 ` | 1179  | 1.2  |
| `scalar F5F Q3F C3  P3 ` | 1181  | 1.9  |
| `scalar F5F Q3F C1R P3 ` | 1201  | 0.9  |
| `scalar F5M Q3F C3  P3 ` | 1222  | 0.5  |
| `scalar F5M Q3F C4F P3 ` | 1223  | 0.4  |
| `scalar F5F Q2X C3  P3 ` | 1223  | 4.4  |
| `scalar F5M Q3F C4M P3 ` | 1226  | 0.6  |
| `scalar F5F Q4M C1X P3 ` | 1227  | 0.4  |
| `scalar F5F Q2X C4F P3 ` | 1231  | 3.9  |
| `scalar F5M Q3F C5F P3 ` | 1237  | 0.4  |
| `scalar F5M Q3F C5M P3 ` | 1244  | 0.3  |
| `scalar F5M Q4M C5M P3 ` | 1245  | 0.7  |
| `scalar F5M Q3F C2R P3 ` | 1253  | 0.3  |
| `scalar F5F Q2X C4M P3 ` | 1270  | 4.1  |
| `scalar F5F Q2X C5M P3 ` | 1277  | 2.5  |
| `scalar F5M Q3F C1X P3 ` | 1288  | 0.5  |
| `scalar F5M Q3F C2X P3 ` | 1289  | 0.5  |
| `scalar F5M Q3F C1R P3 ` | 1301  | 1.0  |
| `scalar F5M Q1  C2X P3 ` | 1331  | 0.4  |
| `scalar F5M Q1  C1X P3 ` | 1332  | 0.4  |
| `scalar F5M Q4F C2X P3 ` | 1376  | 0.4  |
| `scalar F5M Q4F C1X P3 ` | 1377  | 0.5  |
| `scalar F5M Q4M C1X P3 ` | 1429  | 0.4  |
| `scalar F5M Q4M C2X P3 ` | 1474  | 0.5  |
| `scalar F5M Q2X C5F P3 ` | 1474  | 1.2  |
| `scalar F4  Q1  C3  P3 ` | 1537  | 0.3  |
| `scalar F4  Q1  C4F P3 ` | 1541  | 0.3  |
| `scalar F4  Q1  C4M P3 ` | 1563  | 0.3  |
| `scalar F4  Q1  C5F P3 ` | 1565  | 0.2  |
| `scalar F5M Q2X C5M P3 ` | 1570  | 1.3  |
| `scalar F4  Q4F C3  P3 ` | 1572  | 0.5  |
| `scalar F4  Q4F C4F P3 ` | 1581  | 0.6  |
| `scalar F4  Q4F C5F P3 ` | 1595  | 0.4  |
| `scalar F4  Q4F C4M P3 ` | 1598  | 0.5  |
| `scalar F5M Q2X C3  P3 ` | 1612  | 2.0  |
| `scalar F5M Q2X C4F P3 ` | 1618  | 1.5  |
| `scalar F4  Q4M C3  P3 ` | 1624  | 0.8  |
| `scalar F4  Q4M C4F P3 ` | 1634  | 0.8  |
| `scalar F4  Q4M C5F P3 ` | 1635  | 0.7  |
| `scalar F5M Q2X C4M P3 ` | 1637  | 2.0  |
| `scalar F4  Q1  C5M P3 ` | 1645  | 0.5  |
| `scalar F4  Q4M C4M P3 ` | 1654  | 0.7  |
| `scalar F4  Q4F C5M P3 ` | 1668  | 0.4  |
| `object F4  Q1  C1X P3 ` | 1670  | 0.2  |
| `scalar F4  Q1  C2X P3 ` | 1689  | 0.2  |
| `scalar F4  Q1  C1X P3 ` | 1691  | 0.5  |
| `scalar F5F Q2X C2X P3 ` | 1727  | 2.8  |
| `scalar F4  Q4M C5M P3 ` | 1729  | 3.3  |
| `scalar F4  Q4F C2X P3 ` | 1739  | 0.7  |
| `scalar F5F Q2X C1X P3 ` | 1741  | 2.0  |
| `scalar F4  Q4F C1X P3 ` | 1745  | 1.3  |
| `scalar F5F Q1  C2R P3 ` | 1748  | 1.4  |
| `scalar F5F Q4F C2R P3 ` | 1749  | 0.6  |
| `scalar F4  Q2X C5F P3 ` | 1774  | 1.0  |
| `scalar F4  Q2X C3  P3 ` | 1781  | 1.5  |
| `scalar F4  Q2X C4F P3 ` | 1792  | 1.3  |
| `scalar F5F Q4M C2R P3 ` | 1796  | 0.5  |
| `scalar F4  Q4M C2X P3 ` | 1801  | 1.2  |
| `scalar F4  Q4M C1X P3 ` | 1802  | 1.0  |
| `scalar F4  Q2X C4M P3 ` | 1805  | 1.3  |
| `scalar F5F Q2R C5F P3 ` | 1817  | 1.1  |
| `scalar F1X Q3F C4M P3 ` | 1839  | 0.1  |
| `scalar F1X Q3F C3  P3 ` | 1841  | 0.2  |
| `scalar F1X Q3F C4F P3 ` | 1841  | 0.1  |
| `scalar F3R Q3F C4M P3 ` | 1843  | 0.3  |
| `scalar F4  Q2X C5M P3 ` | 1856  | 0.9  |
| `scalar F1X Q3F C2R P3 ` | 1861  | 0.2  |
| `scalar F1X Q3F C5F P3 ` | 1862  | 0.1  |
| `scalar F1X Q3F C1X P3 ` | 1872  | 0.1  |
| `scalar F1X Q3F C2X P3 ` | 1873  | 0.1  |
| `scalar F1X Q3F C5M P3 ` | 1879  | 0.2  |
| `scalar F3R Q3F C2R P3 ` | 1886  | 0.4  |
| `scalar F1X Q3F C1R P3 ` | 1894  | 0.1  |
| `scalar F3R Q3F C5M P3 ` | 1911  | 0.6  |
| `scalar F3R Q3F C5F P3 ` | 1911  | 0.5  |
| `scalar F2R Q3F C4M P3 ` | 1913  | 0.4  |
| `scalar F2R Q3F C5M P3 ` | 1938  | 0.4  |
| `scalar F5F Q2R C5M P3 ` | 1956  | 2.0  |
| `scalar F5F Q2R C4M P3 ` | 1966  | 1.1  |
| `scalar F2R Q3F C3  P3 ` | 1968  | 0.6  |
| `scalar F2R Q3F C5F P3 ` | 1972  | 0.6  |
| `scalar F4  Q2X C1X P3 ` | 1976  | 0.9  |
| `scalar F4  Q2X C2X P3 ` | 1977  | 1.1  |
| `scalar F5F Q2R C4F P3 ` | 1978  | 2.2  |
| `scalar F5M Q1  C2R P3 ` | 1986  | 0.6  |
| `scalar F5F Q4F C1R P3 ` | 1987  | 0.4  |
| `scalar F5F Q2R C3  P3 ` | 1988  | 2.1  |
| `scalar F2R Q3F C2X P3 ` | 2001  | 1.1  |
| `scalar F2R Q3F C4F P3 ` | 2005  | 0.7  |
| `scalar F5F Q1  C1R P3 ` | 2011  | 1.2  |
| `scalar F5F Q4M C1R P3 ` | 2027  | 0.3  |
| `scalar F5M Q2X C2X P3 ` | 2027  | 2.8  |
| `scalar F3R Q3F C3  P3 ` | 2030  | 1.1  |
| `scalar F5M Q2X C1X P3 ` | 2034  | 2.8  |
| `scalar F3R Q3F C1X P3 ` | 2045  | 1.4  |
| `scalar F3R Q3F C2X P3 ` | 2047  | 1.3  |
| `scalar F2R Q3F C1X P3 ` | 2055  | 0.8  |
| `scalar F5M Q4M C2R P3 ` | 2089  | 1.1  |
| `scalar F4  Q1  C2R P3 ` | 2090  | 0.4  |
| `scalar F5M Q2R C5F P3 ` | 2112  | 0.4  |
| `scalar F2R Q3F C1R P3 ` | 2114  | 1.2  |
| `scalar F3X Q3F C3  P3 ` | 2131  | 0.6  |
| `scalar F3X Q3F C4M P3 ` | 2135  | 0.6  |
| `scalar F2X Q3F C4M P3 ` | 2136  | 0.8  |
| `scalar F3R Q3F C1R P3 ` | 2138  | 1.1  |
| `scalar F4  Q4F C2R P3 ` | 2141  | 0.6  |
| `scalar F3R Q3F C4F P3 ` | 2153  | 1.3  |
| `scalar F2X Q3F C3  P3 ` | 2155  | 1.0  |
| `scalar F2X Q3F C5M P3 ` | 2160  | 0.2  |
| `scalar F4  Q2R C5F P3 ` | 2160  | 0.5  |
| `scalar F4  Q2R C4F P3 ` | 2161  | 0.5  |
| `scalar F4  Q2R C3  P3 ` | 2162  | 0.5  |
| `scalar F2R Q3F C2R P3 ` | 2168  | 1.1  |
| `scalar F3X Q3F C5M P3 ` | 2170  | 0.7  |
| `scalar F3X Q3F C2R P3 ` | 2177  | 0.3  |
| `scalar F2X Q3F C5F P3 ` | 2178  | 0.3  |
| `scalar F5M Q2R C3  P3 ` | 2179  | 0.6  |
| `scalar F4  Q2R C4M P3 ` | 2180  | 0.5  |
| `scalar F3X Q3F C5F P3 ` | 2180  | 0.2  |
| `scalar F5M Q2R C4F P3 ` | 2196  | 0.6  |
| `scalar F5M Q4F C2R P3 ` | 2199  | 0.5  |
| `scalar F5M Q2R C5M P3 ` | 2200  | 0.4  |
| `scalar F2X Q3F C2R P3 ` | 2201  | 1.4  |
| `scalar F3X Q3F C1R P3 ` | 2204  | 0.4  |
| `scalar F4  Q4M C2R P3 ` | 2205  | 0.9  |
| `scalar F3X Q3F C4F P3 ` | 2207  | 0.7  |
| `scalar F4  Q1  C1R P3 ` | 2214  | 0.6  |
| `scalar F5M Q2R C4M P3 ` | 2216  | 0.7  |
| `scalar F2X Q3F C4F P3 ` | 2221  | 0.9  |
| `scalar F5M Q1  C1R P3 ` | 2228  | 0.5  |
| `scalar F4  Q2R C5M P3 ` | 2246  | 0.6  |
| `scalar F3X Q3F C2X P3 ` | 2255  | 0.5  |
| `scalar F3X Q3F C1X P3 ` | 2256  | 0.5  |
| `scalar F2X Q3F C1R P3 ` | 2258  | 1.1  |
| `object F4  Q1  C1R P3 ` | 2263  | 0.2  |
| `scalar F2X Q3F C2X P3 ` | 2295  | 1.2  |
| `scalar F4  Q4F C1R P3 ` | 2298  | 1.2  |
| `scalar F2X Q3F C1X P3 ` | 2301  | 0.9  |
| `scalar F5M Q4F C1R P3 ` | 2311  | 1.4  |
| `scalar F5M Q4M C1R P3 ` | 2335  | 0.3  |
| `scalar F4  Q4M C1R P3 ` | 2341  | 0.8  |
| `scalar F4  Q2X C2R P3 ` | 2359  | 0.3  |
| `scalar F4  Q2R C2X P3 ` | 2413  | 0.3  |
| `scalar F4  Q2R C1X P3 ` | 2419  | 0.6  |
| `scalar F5F Q2R C1X P3 ` | 2451  | 2.4  |
| `scalar F5F Q2X C2R P3 ` | 2478  | 7.3  |
| `scalar F4  Q2X C1R P3 ` | 2493  | 0.3  |
| `scalar F5F Q2R C2X P3 ` | 2494  | 4.2  |
| `scalar F5M Q2R C1X P3 ` | 2634  | 0.6  |
| `scalar F5M Q2R C2X P3 ` | 2634  | 0.5  |
| `scalar F5M Q2X C2R P3 ` | 2661  | 1.0  |
| `scalar F4  Q2R C2R P3 ` | 2781  | 0.6  |
| `scalar F5F Q2X C1R P3 ` | 2789  | 3.9  |
| `scalar F5M Q2X C1R P3 ` | 2895  | 0.9  |
| `scalar F4  Q2R C1R P3 ` | 2913  | 0.4  |
| `scalar F5F Q2R C2R P3 ` | 3194  | 2.5  |
| `scalar F1R Q3M C3  P3 ` | 3223  | 0.3  |
| `scalar F1R Q3M C4M P3 ` | 3225  | 0.2  |
| `scalar F1R Q3M C4F P3 ` | 3231  | 0.2  |
| `scalar F1R Q3M C2R P3 ` | 3254  | 0.2  |
| `scalar F1R Q3M C5F P3 ` | 3259  | 0.3  |
| `scalar F1R Q3M C5M P3 ` | 3274  | 0.3  |
| `scalar F1R Q3M C2X P3 ` | 3277  | 0.3  |
| `scalar F5M Q2R C2R P3 ` | 3278  | 0.5  |
| `scalar F1R Q3M C1X P3 ` | 3278  | 0.2  |
| `scalar F1R Q3M C1R P3 ` | 3298  | 0.3  |
| `scalar F5F Q2R C1R P3 ` | 3422  | 2.9  |
| `scalar F5M Q2R C1R P3 ` | 3572  | 0.3  |
| `scalar F1R Q3F C3  P3 ` | 4196  | 0.2  |
| `scalar F1R Q3F C4M P3 ` | 4197  | 0.2  |
| `scalar F1R Q3F C2R P3 ` | 4219  | 0.2  |
| `scalar F1R Q3F C5M P3 ` | 4236  | 0.4  |
| `scalar F1R Q3F C4F P3 ` | 4245  | 0.4  |
| `scalar F1R Q3F C1R P3 ` | 4258  | 0.2  |
| `scalar F1R Q3F C5F P3 ` | 4289  | 0.2  |
| `scalar F1R Q3F C1X P3 ` | 4315  | 0.2  |
| `scalar F1R Q3F C2X P3 ` | 4319  | 0.2  |
| `scalar F1X Q1  C4F P3 ` | 6977  | 0.4  |
| `scalar F1X Q4F C4M P3 ` | 6994  | 0.2  |
| `scalar F1X Q4M C4M P3 ` | 7258  | 1.2  |
| `scalar F1X Q1  C5F P3 ` | 7329  | 0.4  |
| `scalar F1X Q1  C4M P3 ` | 7411  | 0.2  |
| `scalar F1X Q4F C5F P3 ` | 7429  | 0.1  |
| `scalar F1X Q4F C4F P3 ` | 7429  | 0.1  |
| `scalar F1X Q4F C3  P3 ` | 7431  | 0.2  |
| `scalar F1X Q1  C5M P3 ` | 7486  | 0.2  |
| `scalar F1X Q4M C5F P3 ` | 7514  | 0.2  |
| `scalar F1X Q4F C5M P3 ` | 7533  | 0.2  |
| `scalar F1X Q4M C4F P3 ` | 7565  | 0.8  |
| `scalar F1X Q2X C3  P3 ` | 7566  | 0.3  |
| `scalar F1X Q4M C3  P3 ` | 7681  | 1.2  |
| `scalar F1X Q1  C3  P3 ` | 7713  | 8.9  |
| `scalar F1X Q4M C5M P3 ` | 7741  | 2.3  |
| `scalar F1X Q4F C2X P3 ` | 7807  | 0.4  |
| `scalar F1X Q4F C1X P3 ` | 7807  | 0.2  |
| `scalar F1X Q2X C5F P3 ` | 7877  | 0.3  |
| `scalar F1X Q2X C5M P3 ` | 7964  | 0.4  |
| `scalar F1X Q2X C4F P3 ` | 8023  | 0.2  |
| `scalar F1X Q4M C2X P3 ` | 8032  | 1.7  |
| `scalar F1X Q2X C4M P3 ` | 8059  | 0.4  |
| `scalar F1X Q4M C1X P3 ` | 8161  | 1.9  |
| `object F1X Q1  C1X P3 ` | 8180  | 0.1  |
| `scalar F1X Q1  C2X P3 ` | 8189  | 1.7  |
| `scalar F1X Q1  C1X P3 ` | 8353  | 2.9  |
| `scalar F1X Q2X C2X P3 ` | 8411  | 0.4  |
| `scalar F1X Q2X C1X P3 ` | 8413  | 0.4  |
| `scalar F1X Q4F C2R P3 ` | 8428  | 0.2  |
| `scalar F1X Q4F C1R P3 ` | 8463  | 0.1  |
| `scalar F1X Q2R C5F P3 ` | 8512  | 0.3  |
| `scalar F1X Q2R C3  P3 ` | 8599  | 0.3  |
| `scalar F1X Q2R C5M P3 ` | 8630  | 0.3  |
| `scalar F1X Q2R C4M P3 ` | 8636  | 0.3  |
| `scalar F1X Q4M C1R P3 ` | 8647  | 8.8  |
| `scalar F1X Q2R C4F P3 ` | 8672  | 0.4  |
| `scalar F1X Q1  C2R P3 ` | 8712  | 1.7  |
| `scalar F1X Q4M C2R P3 ` | 8848  | 2.3  |
| `scalar F1X Q2X C2R P3 ` | 9094  | 0.3  |
| `object F1X Q1  C1R P3 ` | 9159  | 0.8  |
| `scalar F1X Q2R C1X P3 ` | 9180  | 0.3  |
| `scalar F1X Q2R C2X P3 ` | 9186  | 0.2  |
| `scalar F1X Q2X C1R P3 ` | 9352  | 0.3  |
| `scalar F1X Q1  C1R P3 ` | 9491  | 2.4  |
| `scalar F1X Q2R C2R P3 ` | 9814  | 0.3  |
| `scalar F1X Q2R C1R P3 ` | 10102 | 0.3  |
| `scalar F1R Q4F C5M P3 ` | 26904 | 0.1  |
| `scalar F1R Q4F C3  P3 ` | 27274 | 0.4  |
| `scalar F1R Q4F C5F P3 ` | 27552 | 2.3  |
| `scalar F1R Q4M C1X P3 ` | 27607 | 0.1  |
| `scalar F1R Q4F C4F P3 ` | 27642 | 1.7  |
| `scalar F1R Q2X C5M P3 ` | 27684 | 1.3  |
| `scalar F1R Q2X C5F P3 ` | 27748 | 0.8  |
| `scalar F1R Q2X C4F P3 ` | 27903 | 1.3  |
| `scalar F1R Q4F C1X P3 ` | 28000 | 1.9  |
| `scalar F1R Q1  C4F P3 ` | 28224 | 1.6  |
| `scalar F1R Q2X C1X P3 ` | 28228 | 0.2  |
| `scalar F1R Q2R C5F P3 ` | 28286 | 0.1  |
| `scalar F1R Q1  C1X P3 ` | 28306 | 1.1  |
| `scalar F1R Q4F C4M P3 ` | 28355 | 0.7  |
| `scalar F1R Q2X C4M P3 ` | 28362 | 2.1  |
| `scalar F1R Q2R C5M P3 ` | 28454 | 0.1  |
| `scalar F1R Q1  C5M P3 ` | 28470 | 0.9  |
| `scalar F1R Q4F C2X P3 ` | 28471 | 1.7  |
| `scalar F1R Q2R C3  P3 ` | 28496 | 0.1  |
| `scalar F1R Q2R C4M P3 ` | 28514 | 0.1  |
| `scalar F1R Q2X C2X P3 ` | 28538 | 2.5  |
| `scalar F1R Q4M C2X P3 ` | 28551 | 4.6  |
| `scalar F1R Q1  C2X P3 ` | 28616 | 2.1  |
| `scalar F1R Q2X C3  P3 ` | 28651 | 2.4  |
| `scalar F1R Q2R C4F P3 ` | 28660 | 0.1  |
| `scalar F1R Q4M C2R P3 ` | 28762 | 0.1  |
| `scalar F1R Q4M C5F P3 ` | 28787 | 1.2  |
| `scalar F1R Q1  C5F P3 ` | 28791 | 1.8  |
| `scalar F1R Q4M C4F P3 ` | 28932 | 1.0  |
| `scalar F1R Q4F C1R P3 ` | 29024 | 0.1  |
| `scalar F1R Q4M C1R P3 ` | 29043 | 0.1  |
| `scalar F1R Q1  C3  P3 ` | 29049 | 2.0  |
| `scalar F1R Q1  C1R P3 ` | 29062 | 1.1  |
| `scalar F1R Q2R C2X P3 ` | 29159 | 0.1  |
| `scalar F1R Q2R C1X P3 ` | 29187 | 0.1  |
| `scalar F1R Q1  C2R P3 ` | 29229 | 1.2  |
| `scalar F1R Q4M C5M P3 ` | 29347 | 1.7  |
| `scalar F1R Q4M C3  P3 ` | 29363 | 0.7  |
| `scalar F1R Q2X C2R P3 ` | 29368 | 0.1  |
| `scalar F1R Q1  C4M P3 ` | 29371 | 1.0  |
| `scalar F1R Q2X C1R P3 ` | 29646 | 0.1  |
| `scalar F1R Q4M C4M P3 ` | 29777 | 2.2  |
| `scalar F1R Q4F C2R P3 ` | 29841 | 1.4  |
| `scalar F1R Q2R C2R P3 ` | 30270 | 0.2  |
| `scalar F1R Q2R C1R P3 ` | 33332 | 3.2  |
| `object F1R Q1  C1X P3 ` | 39766 | 0.1  |
| `object F1R Q1  C1R P3 ` | 41333 | 1.4  |

Reference through `minizinc` using [bench.py](/bench.py) on [model.mzn](/arc/puzzles/chain/zinc/model.mzn) with `n = 600`. Statistics from [zinc-bench.csv](/puzzles/chain/zinc-bench.csv):

| name      | mean | stdv |
|-----------|------|------|
| `Gecode`  | 3    | 11.3 |
| `Chuffed` | 22   | 9.6  |
| `CP-SAT`  | 108  | 1.5  |

## VTune UARCH Exploration

### Kernel

```
Elapsed Time:                          3.722s
Clockticks:                   18,298,078,000
Instructions Retired:         75,292,206,000
CPI Rate:                              0.243
MUX Reliability:                       0.967

Retiring:                              69.0%

Front-End Bound:                        7.0%
  Front-End Latency:                    2.5%
  Front-End Bandwidth:                  4.5%

Bad Speculation:                        7.3%
  Branch Mispredict:                   10.1%

Back-End Bound:                        16.7%
  Memory Bound:                         6.6%
    L1 Bound:                           0.0%
    L2 Bound:                           4.5%
    L3 Bound:                           0.3%
    DRAM Bound:                         6.5%
    Store Bound:                        0.1%
  Core Bound:                          10.1%
    Divider:                            0.0%
    Serializing Operations:             2.3%
    Port Utilization:                  19.1%
```

### Gecode

```
Elapsed Time:                          2.126s
Clockticks:                   11,397,428,000
Instructions Retired:         23,703,296,000
CPI Rate:                              0.481
MUX Reliability:                       0.906

Retiring:                              47.4%

Front-End Bound:                       42.7%
  Front-End Latency:                   18.9%
  Front-End Bandwidth:                 23.8%

Bad Speculation:                        0.0%
  Branch Mispredict:                    8.9%

Back-End Bound:                        35.4%
  Memory Bound:                        17.7%
    L1 Bound:                          12.8%
    L2 Bound:                           2.1%
    L3 Bound:                           3.6%
    DRAM Bound:                         5.7%
    Store Bound:                        0.5%
  Core Bound:                          17.7%
    Divider:                            0.7%
    Serializing Operations:            18.2%
    Port Utilization:                  23.9%
```

### Chuffed

```
Elapsed Time:                         13.981s
Clockticks:                   68,531,316,000
Instructions Retired:        167,034,164,000
CPI Rate:                              0.410
MUX Reliability:                       0.980

Retiring:                              41.7%

Front-End Bound:                       22.9%
  Front-End Latency:                   11.4%
  Front-End Bandwidth:                 11.5%

Bad Speculation:                        6.9%
  Branch Mispredict:                    4.7%

Back-End Bound:                        28.6%
  Memory Bound:                        19.4%
    L1 Bound:                          14.5%
    L2 Bound:                           0.5%
    L3 Bound:                           1.7%
    DRAM Bound:                        10.0%
    Store Bound:                        1.7%
  Core Bound:                           9.1%
```

### CP-Sat

```
Elapsed Time:                          4.853s
Clockticks:                   23,818,598,000
Instructions Retired:         62,182,718,000
CPI Rate:                              0.383
MUX Reliability:                       0.939

Retiring:                              41.9%

Front-End Bound:                       30.3%
  Front-End Latency:                   18.8%
  Front-End Bandwidth:                 11.5%

Bad Speculation:                        7.4%
  Branch Mispredict:                    3.7%

Back-End Bound:                        20.4%
  Memory Bound:                        13.7%
    L1 Bound:                          11.1%
    L2 Bound:                           0.8%
    L3 Bound:                           2.2%
    DRAM Bound:                         2.7%
    Store Bound:                        0.2%
  Core Bound:                           6.7%
    Divider:                            0.3%
    Serializing Operations:            10.0%
    Port Utilization:                  20.3%
```

