#Deaths

Death generally follows a sharp downward curve in the first year of life, followed
by a slow and steady rise, reaching nearly 100% around age 120. To simulate this in
small populations without the introduction of randomization, ~~I will cause population
to drop to zero if the total population is 1, and the chance of death is over 50%.~~

If a non-whole number of people die in a year, I should just carry that number over to
the next year, just like with births, so that the statistics don't become unnaturally
skewed down. It's not perfect, since it's going to assume a very gradual series of deaths,
but it's better than actually rolling the dice, since we want reproducible output.

At some point, we'll want to tie deaths into sub-demographics in the population.
For (a horrible) example, in a given year, there will be more poor people dead than
rich. However, that will be an inverted relationship. Deaths will occur first, then
any demographic shifts will be calculated from there.
