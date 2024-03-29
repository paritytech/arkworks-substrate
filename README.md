# Arkworks Sustrate Extensions

Specializations of the crates defined in [arkworks-ext](https://github.com/paritytech/ark-substrate)
ready to be used in the [Substrate](https://github.com/paritytech/substrate) runtime.

## Benchmark results

| extrinsic                               |  arkworkrs(µs)[^1]  |ark-substrate(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |  native(µs)[^5] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)                     |    23335.84      |    3569.35      |${\color{green}\bf 6.54 \boldsymbol{\times}}$|    190.80       |      3440       | 
| bls12_381_pairing                                    |    9092.61       |    1390.80      |${\color{green}\bf 6.54 \boldsymbol{\times}}$|    24.64        |      1270       |
| bls12_381_msm_g1, 10 arguments                       |    6921.99       |    949.58       |${\color{green}\bf 7.29 \boldsymbol{\times}}$|    50.07        |      568.89     |
| bls12_381_msm_g1, 1000 arguments                     |    194969.80     |    30158.23     |${\color{green}\bf 6.46 \boldsymbol{\times}}$|    2169.47      |      10750      |
| bls12_381_msm_g2, 10 arguments                       |    21513.87      |    2870.33      |${\color{green}\bf 7.57 \boldsymbol{\times}}$|    50.06        |      1600       |
| bls12_381_msm_g2, 1000 arguments                     |    621769.22     |    100801.74    |${\color{green}\bf 7.50 \boldsymbol{\times}}$|    3640.63      |      31900      |
| bls12_381_mul_projective_g1                          |    486.34        |    75.01        |${\color{green}\bf 6.48 \boldsymbol{\times}}$|    11.94        |      45.59      |
| bls12_381_mul_affine_g1                              |    420.01        |    79.26        |${\color{green}\bf 5.30 \boldsymbol{\times}}$|    11.11        |      38.74      |
| bls12_381_mul_projective_g2                          |    1498.84       |    210.50       |${\color{green}\bf 7.12 \boldsymbol{\times}}$|    14.63        |      146.93    |
| bls12_381_mul_affine_g2                              |    1234.92       |    214.00       |${\color{green}\bf 5.77 \boldsymbol{\times}}$|    13.17        |      123.68     |
| bls12_377_pairing                                    |    8904.20       |    1449.52      |${\color{green}\bf 6.14 \boldsymbol{\times}}$|    25.88        |      1470       |
| bls12_377_msm_g1, 10 arguments                       |    6592.47       |    902.50       |${\color{green}\bf 7.30 \boldsymbol{\times}}$|    29.20        |      582.19    | 
| bls12_377_msm_g1, 1000 arguments                     |    191793.87     |    28828.95     |${\color{green}\bf 6.65 \boldsymbol{\times}}$|    1307.62      |      11000      |
| bls12_377_msm_g2, 10 arguments                       |    22509.51      |    3251.84      |${\color{green}\bf 6.92 \boldsymbol{\times}}$|    35.06        |      1860       |
| bls12_377_msm_g2, 1000 arguments                     |    632339.00     |    94521.78     |${\color{green}\bf 6.69 \boldsymbol{\times}}$|    2556.48      |      36020      |
| bls12_377_mul_projective_g1                          |    424.21        |    65.68        |${\color{green}\bf 6.46 \boldsymbol{\times}}$|    11.76        |      46.54      |
| bls12_377_mul_affine_g1                              |    363.85        |    65.68        |${\color{green}\bf 5.54 \boldsymbol{\times}}$|    10.50        |      39.81      |
| bls12_377_mul_projective_g2                          |    1339.39       |    212.20       |${\color{green}\bf 6.31 \boldsymbol{\times}}$|    14.56        |      167.91     |
| bls12_377_mul_affine_g2                              |    1122.08       |    208.74       |${\color{green}\bf 5.38 \boldsymbol{\times}}$|    13.08        |      141.49     |
| bw6_761_pairing                                      |    52065.18      |    6791.27      |${\color{green}\bf 7.67 \boldsymbol{\times}}$|    34.70        |      6780       |
| bw6_761_msm_g1, 10 arguments                         |    47050.21      |    5559.53      |${\color{green}\bf 8.46 \boldsymbol{\times}}$|    67.79        |      2760       |
| bw6_761_msm_g1, 1000 arguments                       |    1167536.06    |    143517.21    |${\color{green}\bf 8.14 \boldsymbol{\times}}$|    4630.95      |      56680      | 
| bw6_761_msm_g2, 10 arguments                         |    41055.89      |    4874.46      |${\color{green}\bf 8.42 \boldsymbol{\times}}$|    58.37        |      2960       |
| bw6_761_msm_g2, 1000 arguments                       |    1209593.25    |    143437.77    |${\color{green}\bf 8.43 \boldsymbol{\times}}$|    4345.36      |      74550      |
| bw6_761_mul_projective_g1                            |    1678.86       |    223.57       |${\color{green}\bf 7.51 \boldsymbol{\times}}$|    27.54        |      221.73     |
| bw6_761_mul_affine_g1                                |    1387.87       |    222.05       |${\color{green}\bf 6.25 \boldsymbol{\times}}$|    27.55        |      183.16     |
| bw6_761_mul_projective_g2                            |    1919.98       |    308.60       |${\color{green}\bf 6.22 \boldsymbol{\times}}$|    26.99        |      221.75     |
| bw6_761_mul_affine_g2                                |    1388.21       |    222.47       |${\color{green}\bf 6.24 \boldsymbol{\times}}$|    21.90        |      184.79     |
| ed_on_bls12_381_bandersnatch_msm_sw, 10 arguments    |    3616.81       |    557.96       |${\color{green}\bf 6.48 \boldsymbol{\times}}$|    21.43        |      457.93     |
| ed_on_bls12_381_bandersnatch_msm_sw, 1000 arguments  |    94473.54      |    16254.32     |${\color{green}\bf 5.81 \boldsymbol{\times}}$|    982.29      |      7460       |
| ed_on_bls12_381_bandersnatch_mul_projective_sw       |    235.38        |    40.70        |${\color{green}\bf 5.78 \boldsymbol{\times}}$|    9.03        |      33.12      |
| ed_on_bls12_381_bandersnatch_mul_affine_sw           |    204.04        |    41.66        |${\color{green}\bf 4.90 \boldsymbol{\times}}$|    8.78        |      29.50     |
| ed_on_bls12_381_bandersnatch_msm_te, 10 arguments    |    5427.77       |    744.74       |${\color{green}\bf 7.29 \boldsymbol{\times}}$|    24.05        |      538.16     |
| ed_on_bls12_381_bandersnatch_msm_te, 1000 arguments  |    106610.20     |    16690.71     |${\color{green}\bf 6.39 \boldsymbol{\times}}$|    1195.35      |      7460       |
| ed_on_bls12_381_bandersnatch_mul_projective_te       |    183.29        |    34.63        |${\color{green}\bf 5.29 \boldsymbol{\times}}$|    9.55        |      24.83      |  
| ed_on_bls12_381_bandersnatch_mul_affine_te           |    181.84        |    33.99        |${\color{green}\bf 5.35 \boldsymbol{\times}}$|    9.50        |      29.47      |
| ed_on_bls12_377_msm, 10 arguments                    |    5304.03       |    700.51       |${\color{green}\bf 7.57 \boldsymbol{\times}}$|    24.02        |      523.27     | 
| ed_on_bls12_377_msm, 1000 arguments                  |    105563.53     |    15757.62     |${\color{green}\bf 6.70 \boldsymbol{\times}}$|    1200.45      |      7370       |
| ed_on_bls12_377_mul_projective                       |    179.54        |    32.72        |${\color{green}\bf 5.49 \boldsymbol{\times}}$|    9.72        |      24.07      |
| ed_on_bls12_377_mul_affine                           |    177.53        |    33.24        |${\color{green}\bf 5.34 \boldsymbol{\times}}$|    9.76        |      23.90      |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library.
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library.
[^3]: speedup by using ark-substrate with host calls, compared to native speed.
[^4]: These extrinsics just receive the arguments, deserialize them without using them and then take a generator
      or zero element of the expected return group, serialize it and return it. 
      **Calling a host call through an extrinsic which does nothing has been benchmarked with 3.98µs**.
[^5]: native execution, computed by [this](https://github.com/achimcc/native-bench-arkworks) repo.

## Usage

Refer to [ark-substrate-examples](https://github.com/davxy/ark-substrate-examples) for some insights.
