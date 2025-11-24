namespace AdventOfCodeMain.DaySolutions;

internal class Day08
{
    public static void Run()
    {
        string[] challengeInput = FileReader.ReadFromFile("Day8.1").Split("\r\n");
        int numRows = challengeInput.Length;
        int numCols = challengeInput[0].Length;

        Dictionary<char, List<(int, int)>> antennaPositions = FindAntennas(challengeInput, numRows, numCols);

        (int part1, int part2) nodeCounts = FindAllNodes(antennaPositions, numRows, numCols);

        Console.WriteLine(nodeCounts);


    }

    private static Dictionary<char, List<(int, int)>> FindAntennas(string[] map, int numRows, int numCols)
    {
        Dictionary<char, List<(int, int)>> antennaPositions = new();
        for (int i = 0; i < numRows * numCols; i++)
        {
            int xPos = i / numCols;
            int yPos = i % numCols;

            char currentLocationChar = map[xPos][yPos];

            if (currentLocationChar != '.')
            {
                if (!antennaPositions.TryGetValue(currentLocationChar, out var positions))
                {
                    positions = new List<(int, int)>();
                    antennaPositions[currentLocationChar] = positions;
                }
                positions.Add((xPos, yPos));
            }
        }
        return antennaPositions;
    }

    private static (int, int) FindAllNodes(Dictionary<char, List<(int, int)>> antennaPositions, int numRows, int numCols)
    {
        HashSet<(int, int)> nodes = new();
        HashSet<(int, int)> resonantHarmonicsNodes = new();
        foreach (var antennaGroup in antennaPositions)
        {
            MarkNodes(antennaGroup.Value, nodes, resonantHarmonicsNodes, numRows, numCols);
        }
        return (nodes.Count, resonantHarmonicsNodes.Count);
    }

    private static void MarkNodes(List<(int row, int col)> positions, HashSet<(int, int)> nodes, HashSet<(int, int)> resonantHarmonicNodes, int numRows, int numCols)
    {
        
        for (int i = 0; i < positions.Count; i++)
        {
            for (int j = i + 1; j < positions.Count; j++) // for each combination (i choose 2, iC2)
            {
                (int, int) vector = SubtractTuples(positions[j], positions[i]);
                (int row, int col) node1 = SubtractTuples(positions[i], vector);
                (int row, int col) node2 = AddTuples(positions[j], vector);

                if (IsWithinBounds(node1, numRows, numCols))
                {
                    nodes.Add(node1);
                }
                if (IsWithinBounds(node2, numRows, numCols))
                {
                    nodes.Add(node2);
                }

                TraverseDirections(positions[i], vector, resonantHarmonicNodes, numRows, numCols);
            }
        }
    }

    private static void TraverseDirections((int row, int col) start, (int row, int col) vector, HashSet<(int, int)> nodes, int numRows, int numCols)
    {
        nodes.Add(start);
        foreach (int direction in new[] { 1, -1 }) // positive and negative directions
        {
            (int row, int col) currentNode = start;
            while (true)
            {
                currentNode = AddTuples(currentNode, (vector.row * direction, vector.col * direction));
                if (IsWithinBounds(currentNode, numRows, numCols))
                {
                    nodes.Add(currentNode);
                }
                else
                {
                    break;
                }
            }
        }
    }

    private static (int, int) AddTuples((int, int) tuple1, (int, int) tuple2) => (tuple1.Item1 + tuple2.Item1, tuple1.Item2 + tuple2.Item2);
    private static (int, int) SubtractTuples((int, int) tuple1, (int, int) tuple2) => (tuple1.Item1 - tuple2.Item1, tuple1.Item2 - tuple2.Item2);
    private static bool IsWithinBounds((int row, int col) position, int numRows, int numCols) => position.row >= 0 && position.row < numRows && position.col >= 0 && position.col < numCols;
}
