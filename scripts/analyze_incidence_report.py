import altair as alt
import numpy as np
import polars as pl

POPULATION = 100
FOI = 0.1


def main() -> None:
    sim = (
        pl.read_csv("incidence.csv")
        .with_columns(
            s=POPULATION - (pl.col("infection_status") == pl.lit("I")).cum_sum(),
            r=(pl.col("infection_status") == pl.lit("R")).cum_sum(),
        )
        .with_columns(i=POPULATION - (pl.col("s") + pl.col("r")))
        .select(["time", "s", "i", "r"])
        .unpivot(index="time")
        .with_columns(source=pl.lit("simulation"))
    )

    # theory
    t = np.linspace(0.0, sim["time"].to_numpy().max(), num=101)
    s = POPULATION * np.exp(-(FOI * t))

    theory = pl.DataFrame({"time": t, "value": s, "variable": "s", "source": "theory"})

    chart = (
        alt.Chart(pl.concat([sim, theory], how="diagonal_relaxed"))
        .mark_line()
        .encode(
            alt.X("time"),
            alt.Y("value"),
            alt.Color("variable"),
            alt.StrokeDash("source"),
        )
    )

    chart.save("incidence.svg")


if __name__ == "__main__":
    main()
