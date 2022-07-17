
<!-- README.md is generated from README.Rmd. Please edit that file -->

# uuid

<!-- badges: start -->
<!-- badges: end -->

This is an example package for using Rust code in R with extendr.

## Installation

You can install the development version of uuid from
[GitHub](https://github.com/) with:

``` r
# install.packages("devtools")
devtools::install_github("qiushiyan/uuid")
```

## Example

Generate a character vector of UUID4 strings of given length:

``` r
library(uuid)
uuid(10, "4")
#>  [1] "4805cfc5-0f9e-49ec-a973-9a2b8ce2d921"
#>  [2] "a06a3a5c-d5f6-4f0a-9b24-c311312cfd43"
#>  [3] "00f7565c-8982-4413-a033-475434651d96"
#>  [4] "a8aaaa34-c6fc-4f9e-b6ba-4c152f6bded9"
#>  [5] "315c06a1-2374-4408-92c2-59536b850ba6"
#>  [6] "b06cf27c-4176-47bc-aae1-517542eb0f16"
#>  [7] "20de8c58-59c3-46b2-9fd1-5da3be2ea028"
#>  [8] "3ea8cd72-3167-4cee-8481-6111cad5a47f"
#>  [9] "fa5ea3bf-95d6-4ade-a1dc-18b8fc8e99b7"
#> [10] "0a183b3c-8667-401d-ad0c-2a8a479e9cf7"
```
