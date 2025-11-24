using System.Diagnostics;

namespace AdventOfCodeMain.DaySolutions
{
    internal class Day04
    {
        public static void Run()
        {
            string challengeInput = FileReader.ReadFromFile("Day4.1");
            string[] lines = challengeInput.Split("\r\n");

            int numRows = lines.Length;
            int numCols = lines[0].Length;
            
            int XMASCounter = Part1(lines, numRows, numCols);

            Stopwatch sw = new Stopwatch();
            sw.Start();
            int X_MASCounter = Part2(lines, numRows, numCols);
            sw.Stop();
            Console.WriteLine($"Elapsed time: {sw.ElapsedMilliseconds}ms");

            Console.WriteLine($"Total XMAS found: {XMASCounter}\nTotal X-MAS found: {X_MASCounter}");
        }

        private static readonly List<(int Row, int Col)> directions = new()
        {
            (-1,  0), // Up
            ( 1,  0), // Down
            ( 0, -1), // Left
            ( 0,  1), // Right
            (-1, -1), // Up-Left
            (-1,  1), // Up-Right
            ( 1, -1), // Down-Left
            ( 1,  1)  // Down-Right
        };

        // Masks for the patterns
        private static readonly char[][][] masks = new char[][][]
        {
            new char[][] { new[] { 'M', '.', 'M' }, new[] { '.', 'A', '.' }, new[] { 'S', '.', 'S' } },
            new char[][] { new[] { 'S', '.', 'S' }, new[] { '.', 'A', '.' }, new[] { 'M', '.', 'M' } },
            new char[][] { new[] { 'M', '.', 'S' }, new[] { '.', 'A', '.' }, new[] { 'M', '.', 'S' } },
            new char[][] { new[] { 'S', '.', 'M' }, new[] { '.', 'A', '.' }, new[] { 'S', '.', 'M' } }
        };

        private static int Part1(string[] lines, int numRows, int numCols)
        {
            string word = "XMAS";
            int matchCounter = 0;

            var stopwatch = new Stopwatch();

            // Test flat loop
            stopwatch.Start();
            for (int pos = 0; pos < numRows * numCols; pos++)
            {
                int row = pos / numCols;
                int col = pos % numCols;

                if (lines[row][col] == word[0])
                {
                    foreach (var direction in directions)
                    {
                        if (IsWordInDirection(lines, row, col, word, direction, numRows, numCols))
                        {
                            matchCounter++;
                        }
                    }
                }
            }

            stopwatch.Stop();
            Console.WriteLine($"Flat loop time: {stopwatch.ElapsedMilliseconds}ms"); // about 1ms

            // Test LINQ version
            stopwatch.Restart();
            var matchCountLINQ = Enumerable.Range(0, numRows)
                .SelectMany(row => Enumerable.Range(0, numCols), (row, col) => new { row, col })
                .Where(pos => lines[pos.row][pos.col] == word[0])
                .Sum(pos => directions.Count(direction =>
                    IsWordInDirection(lines, pos.row, pos.col, word, direction, numRows, numCols)));

            stopwatch.Stop();
            Console.WriteLine($"LINQ time: {stopwatch.ElapsedMilliseconds}ms"); // about 8ms

            return matchCounter;
        }

        public static int Part2(string[] lines, int numRows, int numCols)
        {
            int matchCounter = 0;
            int totalCells = (numRows - 2) * (numCols - 2); // Total valid 3x3 windows

            for (int pos = 0; pos < totalCells; pos++)
            {
                int row = pos / (numCols - 2); // Calculate the row
                int col = pos % (numCols - 2); // Calculate the column

                foreach (var mask in masks)
                {
                    if (MatchesMask(lines, row, col, mask))
                    {
                        matchCounter++;
                        break;
                    }
                }
            }
            return matchCounter;
        }

        private static bool MatchesMask(string[] grid, int startRow, int startCol, char[][] mask)
        {
            for (int row = 0; row < 3; row++)
            {
                for (int col = 0; col < 3; col++)
                {
                    // Skip the character check if the mask has '.'
                    if (mask[row][col] != '.' && grid[startRow + row][startCol + col] != mask[row][col])
                    {
                        return false;
                    }
                }
            }
            return true;
        }


        private static bool IsWordInDirection(string[] lines, int startRow, int startCol, string word, (int, int) direction, int numRows, int numCols)
        {
            for (int i = 1; i < word.Length; i++)
            {
                int newRow = startRow + i * direction.Item1;
                int newCol = startCol + i * direction.Item2;

                // Check boundaries
                if (newRow < 0 || newRow >= numRows || newCol < 0 || newCol >= numCols)
                    return false;

                if (lines[newRow][newCol] != word[i])
                    return false;
            }
            return true;
        }
    }
}
