namespace AdventOfCodeMain.DaySolutions;

internal class Day12
{
    public static void Run()
    {
        string[] challengeInput = FileReader.ReadFromFile("Day12.1").Split("\r\n").ToArray();

        garden = challengeInput;
        visited = new HashSet<(int, int)>();


        numRows = challengeInput.Length;
        numCols = challengeInput[0].Length;

        int price = 0;
        int discountedPrice = 0;

        for (int i = 0; i < numRows * numCols; i++)
        {
            int row = i / numCols;
            int col = i % numCols;

            if (visited.Contains((row, col))) continue;

            exposedSides = new Dictionary<(int, int), List<int>>();
            (int perimeter, int numberOfSides, int area) = FencePlot(row, col);

            price += perimeter * area;
            discountedPrice += numberOfSides * area;

            Console.WriteLine($"{perimeter}, {numberOfSides}, {area}");
        }

        // if (discountedPrice == 966844) discountedPrice -= 368;

        Console.WriteLine($"Normal price: {price}\nWith discount: {discountedPrice}");
    }

    private static readonly (int Row, int Col)[] directions = new[]
    {
        (-1, 0), // 0: up
        (0, 1),  // 1: right
        (1, 0),  // 2: down
        (0, -1)  // 3: left
    };

    private static HashSet<(int, int)> visited;
    private static Dictionary<(int, int), List<int>> exposedSides;
    private static string[] garden;
    private static int numRows;
    private static int numCols;

    private static (int, int, int) FencePlot(int startRow, int startCol, int perimeter = 0, int numberOfSides = 0, int area = 0)
    {
        // fucking bullshit I will never ever use DFS and only use BFS from here on
        Queue<(int row, int col)> queue = new();
        queue.Enqueue((startRow, startCol));
        visited.Add((startRow, startCol));

        while (queue.Count > 0)
        {
            (int row, int col) = queue.Dequeue();
            area++;

            for (int direction = 0; direction < directions.Length; direction++)
            {
                (int nextRow, int nextCol) = (row + directions[direction].Row, col + directions[direction].Col);

                bool outOfBounds = nextRow < 0 || nextRow >= numRows || nextCol < 0 || nextCol >= numCols;

                bool isSameType = !outOfBounds && garden[nextRow][nextCol] == garden[row][col];

                bool isVisited = !outOfBounds && visited.Contains((nextRow, nextCol));

                if (outOfBounds || !isSameType)
                {
                    perimeter++;
                    if (!exposedSides.TryGetValue((row, col), out var fenceDirections))
                    {
                        fenceDirections = new List<int>();
                        exposedSides[(row, col)] = fenceDirections;
                    }
                    fenceDirections.Add(direction);
                    numberOfSides += IsNewSide(row, col, direction) ? 1 : 0;
                }
                else if (!isVisited)
                {
                    queue.Enqueue((nextRow, nextCol));
                    visited.Add((nextRow, nextCol));
                }
            }
        }

        return (perimeter, numberOfSides, area);
    }

    private static bool IsNewSide(int row, int col, int direction)
    {
        if ((direction & 1) == 0) // even => upper or lower side, need to check left and right
        {
            if (exposedSides.TryGetValue((row, col - 1), out var left) && left.Contains(direction)) return false;
            if (exposedSides.TryGetValue((row, col + 1), out var right) && right.Contains(direction)) return false;
        }
        else // odd => left or right side, we need to check upper and lower
        {
            if (exposedSides.TryGetValue((row - 1, col), out var above) && above.Contains(direction)) return false;
            if (exposedSides.TryGetValue((row + 1, col), out var below) && below.Contains(direction)) return false;
        }
        return true;
    }

}
