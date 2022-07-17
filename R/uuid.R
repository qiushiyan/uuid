#' Generate UUID strings
#'
#' Generate a character vector of UUID strings of given length
#'
#' @param n Output length
#' @param version UUID version
#' @return A character vector
uuid <- function(n = 1, version = c("1", "3", "4", "5")) {
  version <- match.arg(version)
  uuid_(n, as.integer(version))
}
