import csv
import plotly.express as px
import plotly.graph_objects as go

MES_CSV_FILE = "mes/mes.csv"

def read_data_from_file(file_path):
    with open(file_path, mode='r', encoding='utf-8') as file:
        reader = csv.DictReader(file, delimiter="|")
        return list(reader)

def get_mes_data(mes_data):
    sizes = []
    std_mes = []
    winograd_mes = []
    opt_winograd_mes = []

    for data in mes_data:
        sizes.append(data["matrix_size"])
        std_mes.append(float(data["std_mult"]))
        winograd_mes.append(float(data["winograd_mult"]))
        opt_winograd_mes.append(float(data["opt_winograd_mult"]))

    return (sizes, std_mes, winograd_mes, opt_winograd_mes)


if __name__ == '__main__':
    mes_data = read_data_from_file(MES_CSV_FILE)
    sizes, std_mes, winograd_mes, opt_winograd_mes = get_mes_data(mes_data)

    fig = go.Figure()

    fig.add_trace(go.Scatter(x=sizes, y=std_mes, mode='lines+markers', name="Стандартное умножение"))
    fig.add_trace(go.Scatter(x=sizes, y=winograd_mes, mode='lines+markers', name="Алгоритм Винограда"))
    fig.add_trace(go.Scatter(x=sizes, y=opt_winograd_mes, mode='lines+markers', name="Оптимизированный алгоритм Винограда"))

    fig.update_layout(go.Layout(
        title="Сравнение алгоритмов умножения матриц",
        xaxis=dict(
            title="Размер квадратной матрицы"
        ),
        yaxis=dict(
            title="Время выполнения (мс)"
        )
    ))

    fig.write_html("mes/mes_plot.html")