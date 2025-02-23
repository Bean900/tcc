import matplotlib.pyplot as plt

teams = [
    (50.09523, 8.66144, "Team 1"),
    (50.11092, 8.68212, "Team 2"),
    (50.11552, 8.68417, "Team 3"),
    (50.11222, 8.65119, "Team 4"),
    (50.11667, 8.66972, "Team 5"),
    (50.11417, 8.67861, "Team 6"),
    (50.10722, 8.66972, "Team 7"),
    (50.11333, 8.66972, "Team 8"),
    (50.11111, 8.68333, "Team 9"),
    #(50.11389, 8.68278, "Team 10"),
    #(50.11028, 8.68278, "Team 11"),
    #(50.11611, 8.68222, "Team 12"),
    #(50.12028, 8.68333, "Team 13"),
    #(50.11833, 8.68222, "Team 14"),
    #(50.11056, 8.68444, "Team 15"),
    #(50.11083, 8.70111, "Team 16"),
   # (50.10639, 8.66944, "Team 17"),
   # (50.11583, 8.68222, "Team 18"),
]

x = [team[0] for team in teams]
y = [team[1] for team in teams]
labels = [team[2] for team in teams]

plt.figure(figsize=(10, 8))
plt.scatter(x, y, color='blue')

for i, label in enumerate(labels):
    plt.text(x[i], y[i], label, fontsize=9, ha='right')

plt.xlabel('latitude')
plt.ylabel('longitude')
plt.title('Position of teams in Frankfurt am Main')

plt.grid(True)
plt.show()