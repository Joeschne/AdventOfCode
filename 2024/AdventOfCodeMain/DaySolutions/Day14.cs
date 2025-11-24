using System.Text.RegularExpressions;

namespace AdventOfCodeMain.DaySolutions;

internal class Day14
{
    public static void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day14.1");

        string pattern = @"p=(\d+),(\d+) v=(-?\d+),(-?\d+)";
        Regex regex = new(pattern);

        const int numCols = 101;
        const int numRows = 103;
        const int seconds = 100;

        var (positions, velocities) = ParseInput(regex, challengeInput);

        int score = CalculatePositions(numCols, numRows, seconds, positions, velocities);
        Console.WriteLine($"Score after 100s: {score}");
        Console.WriteLine("Press any key to continue to Part 2");
        Console.ReadKey(true);

        Console.WriteLine("Finding Christmas tree...");
        FindChristmasTree(numCols, numRows, positions, velocities);
    }

    private static (List<(int col, int row)> positions, List<(int col, int row)> velocities) ParseInput(Regex regex, string input)
    {
        var positions = new List<(int col, int row)>();
        var velocities = new List<(int col, int row)>();
        foreach (Match match in regex.Matches(input))
        {
            positions.Add((int.Parse(match.Groups[1].Value), int.Parse(match.Groups[2].Value)));
            velocities.Add((int.Parse(match.Groups[3].Value), int.Parse(match.Groups[4].Value)));
        }
        return (positions, velocities);
    }

    private static int CalculatePositions(int numCols, int numRows, int seconds, List<(int col, int row)> positions, List<(int col, int row)> velocities)
    {
        int halfCols = numCols / 2,
            halfRows = numRows / 2;

        Dictionary<int, int> quadrants = new()
        {
            { 1, 0 },
            { 2, 0 },
            { 3, 0 },
            { 4, 0 }
        };

        for (int i = 0; i < positions.Count; i++)
        {
            int finalCol = ((positions[i].col + seconds * velocities[i].col) % numCols + numCols) % numCols;
            int finalRow = ((positions[i].row + seconds * velocities[i].row) % numRows + numRows) % numRows;


            if (finalCol < halfCols && finalRow < halfRows)
                quadrants[1]++; // top-left
            else if (finalCol > halfCols && finalRow < halfRows)
                quadrants[2]++; // top-right
            else if (finalCol < halfCols && finalRow > halfRows)
                quadrants[3]++; // bottom-left
            else if (finalCol > halfCols && finalRow > halfRows)
                quadrants[4]++; // bottom-right
        }
        return quadrants[1] * quadrants[2] * quadrants[3] * quadrants[4];
    }

    private static void FindChristmasTree(int numCols, int numRows, List<(int col, int row)> positions, List<(int col, int row)> velocities)
    {
        const int subGridSize = 4;
        const double entropyThresholdRatio = 0.04; // for this I just had to play around until something worked
        int numberOfRobots = positions.Count;
        double minSubGridEntropy = Math.Log(subGridSize * subGridSize, 2) * entropyThresholdRatio;

        int seconds = 0;
        while (true)
        {
            var robotPositions = UpdateRobotPositions(numCols, numRows, positions, velocities);

            int highEntropyCount = CalculateHighEntropySubgrids(numCols, numRows, subGridSize, robotPositions, numberOfRobots, minSubGridEntropy);

            if (highEntropyCount >= 2)
            {
                DisplayGrid(numCols, numRows, robotPositions);
                Console.WriteLine($"Detected {highEntropyCount} high-entropy subgrids at {seconds} seconds. Press 'q' to quit or any other key to continue...");
                if (Console.ReadKey(true).KeyChar is 'q' or 'Q') break;
            }
            seconds++;
        }

        Console.WriteLine($"Stopped at {seconds} seconds.");
    }

    private static Dictionary<(int col, int row), int> UpdateRobotPositions(int numCols, int numRows, List<(int col, int row)> positions, List<(int col, int row)> velocities)
    {
        var robotPositions = new Dictionary<(int, int), int>();
        for (int i = 0; i < positions.Count; i++)
        {
            var pos = positions[i];
            if (!robotPositions.TryAdd(pos, 1))
                robotPositions[pos]++;

            positions[i] = (
                (pos.col + velocities[i].col + numCols) % numCols,
                (pos.row + velocities[i].row + numRows) % numRows
            );
        }
        return robotPositions;
    }

    private static int CalculateHighEntropySubgrids(int numCols, int numRows, int subGridSize, Dictionary<(int, int), int> robotPositions, int totalRobots, double minSubGridEntropy)
    {
        int highEntropyCount = 0;
        for (int startRow = 0; startRow < numRows; startRow += subGridSize)
        {
            for (int startCol = 0; startCol < numCols; startCol += subGridSize)
            {
                double subGridEntropy = CalculateSubGridEntropy(startRow, startCol, subGridSize, numCols, numRows, robotPositions, totalRobots);
                if (subGridEntropy > minSubGridEntropy)
                    highEntropyCount++;
            }
        }
        return highEntropyCount;
    }

    private static double CalculateSubGridEntropy(int startRow, int startCol, int subGridSize, int numCols, int numRows, Dictionary<(int, int), int> robotPositions, int totalRobots)
    {
        double entropy = 0;
        for (int row = startRow; row < startRow + subGridSize && row < numRows; row++)
        {
            for (int col = startCol; col < startCol + subGridSize && col < numCols; col++)
            {
                if (robotPositions.TryGetValue((col, row), out int count) && count > 0)
                {
                    double probability = (double)count / totalRobots;
                    entropy += -probability * Math.Log(probability, 2);
                }
            }
        }
        return entropy;
    }

    private static void DisplayGrid(int numCols, int numRows, Dictionary<(int, int), int> robotPositions)
    {
        Console.Clear();
        for (int row = 0; row < numRows; row++)
        {
            for (int col = 0; col < numCols; col++)
            {
                Console.Write(robotPositions.TryGetValue((col, row), out int count) && count > 0
                    ? (count > 9 ? '*' : count.ToString())
                    : '.');
            }
            Console.WriteLine();
        }
    }
}
