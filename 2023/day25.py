import random


def part1(input):
    graph = {}
    for line in input.splitlines():
        name, children = line.split(":")
        children = children.split()

        if name not in graph:
            graph[name] = []

        for child in children:
            if child not in graph:
                graph[child] = []
            graph[child].append(name)
            graph[name].append(child)

    times_visited = {}
    for _ in range(100):
        p1, p2 = random.sample(graph.keys(), 2)

        queue = [[p1]]
        visited = set()

        while queue:
            path = queue.pop(0)

            node = path[-1]

            if node == p2:
                break

            if node in visited:
                continue

            visited.add(node)

            for child in graph[node]:
                new_path = list(path)
                new_path.append(child)
                queue.append(new_path)

        for n1, n2 in zip(path, path[1:]):
            edge_name = tuple(sorted([n1, n2]))
            times_visited[edge_name] = times_visited.get(edge_name, 0) + 1

    top_three = sorted(((v, k) for k, v in times_visited.items()), reverse=True)[:3]
    queue = [next(iter(graph.keys()))]
    flooded = set()

    while queue:
        node = queue.pop(0)

        if node in flooded:
            continue

        flooded.add(node)

        for child in graph[node]:
            edge_name = tuple(sorted([node, child]))
            if edge_name not in [x[1] for x in top_three]:
                queue.append(child)

    num_all = len(graph.keys())
    num_flooded = len(flooded)

    return f"{(num_all - num_flooded) * num_flooded} (if you get 0, run again)"


def part2(input):
    return "Merry Christmas!"
