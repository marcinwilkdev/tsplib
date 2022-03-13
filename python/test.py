import tsplib

edges = tsplib.parse_file("lower_diag_row")
print(edges)

edges = tsplib.parse_file("full_matrix")
print(edges)

edges = tsplib.parse_file("euc_2d")
print(edges)
