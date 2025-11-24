namespace AdventOfCodeMain.DaySolutions;

internal class Day10
{
    public static void Run()
    {
        string[] challengeInput = FileReader.ReadFromFile("Day10.1").Split("\r\n");

        int numRows = challengeInput.Length;
        int numCols = challengeInput[0].Length;

        int validTrails = 0;
        int trailHeadScores = 0;

        for (int i = 0; i < numRows * numCols; i++)
        {
            int row = i / numCols;
            int col = i % numCols;

            if (challengeInput[row][col] == '0')
            {
                // first method is a BFS implementation, second one a DFS. Could be used interchangably, part1 simply needs "visitedTracking", part2 doesn't
                trailHeadScores += CalculateTrailHeadScore(row, col, challengeInput, numRows, numCols);
                validTrails = FindHikingTrails(row, col, challengeInput, numRows, numCols, validTrails);
            }
        }
        Console.WriteLine($"Valid Trails: {validTrails}\nTrailHeadScores: {trailHeadScores}");
    }

    private static readonly (int Row, int Col)[] directions = new[]
    {
        (-1, 0), // 0: up
        (0, 1),  // 1: right
        (1, 0),  // 2: down
        (0, -1)  // 3: left
    };

    private static int FindHikingTrails(int row, int col, string[] challengeInput, int numRows, int numCols, int trailheads)
    {
        char currentChar = challengeInput[row][col];
        if (currentChar == '9') return ++trailheads;

        foreach (var direction in directions) // all directions
        {
            (int nextRow, int nextCol) = AddTuples((row, col), direction);

            if (nextRow >= 0 && nextRow < numRows && nextCol >= 0 && nextCol < numCols &&
                currentChar + 1 == challengeInput[nextRow][nextCol])
            {
                trailheads = FindHikingTrails(nextRow, nextCol, challengeInput, numRows, numCols, trailheads);
            }
        }

        return trailheads;
    }

    private static int CalculateTrailHeadScore(int row, int col, string[] challengeInput, int numRows, int numCols)
    {
        var visited = new HashSet<(int, int)>();
        int reachableNines = 0;
        var queue = new Queue<(int, int)>();

        queue.Enqueue((row, col));
        visited.Add((row, col));

        while (queue.Count > 0)
        {
            var (currentRow, currentCol) = queue.Dequeue();
            char currentHeight = challengeInput[currentRow][currentCol];

            if (currentHeight == '9')
            {
                reachableNines++;
                continue;
            }

            foreach (var direction in directions) // all directions
            {
                (int nextRow, int nextCol) = AddTuples((currentRow, currentCol), direction);

                if (nextRow >= 0 && nextRow < numRows && nextCol >= 0 && nextCol < numCols &&
                    !visited.Contains((nextRow, nextCol)) &&
                    challengeInput[nextRow][nextCol] == currentHeight + 1)
                {
                    visited.Add((nextRow, nextCol));
                    queue.Enqueue((nextRow, nextCol));
                }
            }
        }

        return reachableNines;
    }

    private static (int, int) AddTuples((int, int) tuple1, (int, int) tuple2) => (tuple1.Item1 + tuple2.Item1, tuple1.Item2 + tuple2.Item2);
}
