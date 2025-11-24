namespace AdventOfCodeMain.DaySolutions;

internal class Day06
{
    public static void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day6.1");
        char[][] maze = challengeInput.Split("\r\n").Select(row => row.ToCharArray()).ToArray(); // initialize maze as char[][] because strings are immutable
        int amountRows = maze.Length - 1;
        int amountCols = maze[0].Length - 1;
        (int, int) startingPosition = FindStartingPosition(maze);
        int currentDirection = 0; // start as '^' => up

        int amountOfPositions = FindAmountOfPositions(maze, startingPosition, currentDirection, amountRows, amountCols);

        int amountOfLoops = FindAmountOfLoops(maze, startingPosition, currentDirection, amountRows, amountCols);

        Console.WriteLine($"Positions: {amountOfPositions}\r\nPossible loops: {amountOfLoops}");
    }

    private static readonly (int Row, int Col)[] directions = new[]
    {
        (-1, 0), // 0: up
        (0, 1),  // 1: right
        (1, 0),  // 2: down
        (0, -1)  // 3: left
    };

    private static int FindAmountOfPositions(char[][] maze, (int row, int col) currentPosition, int currentDirection, int amountRows, int amountCols)
    {
        maze[currentPosition.row][currentPosition.col] = 'X'; // visited the start
        while (true)
        {
            (int row, int col) nextPosition = AddTuples(currentPosition, directions[currentDirection]);

            bool isExit = nextPosition.row < 0 || nextPosition.row > amountRows ||
                          nextPosition.col < 0 || nextPosition.col > amountCols;
            if (isExit) return maze.Sum(row => row.Count(cell => cell == 'X'));

            bool isBlocked = maze[nextPosition.row][nextPosition.col] == '#';
            if (isBlocked)
            {
                currentDirection = Turn(currentDirection);
            }
            else
            {
                currentPosition = nextPosition;
                maze[nextPosition.row][nextPosition.col] = 'X'; // visited
            }
        }
    }

    private static int FindAmountOfLoops(char[][] maze, (int row, int col) currentPosition, int currentDirection, int amountRows, int amountCols)
    {
        int amountOfLoops = 0;
        var intialPosition = currentPosition;

        for (int obstaclePos = 0; obstaclePos < amountRows * amountCols; obstaclePos++)
        {
            int obstacleRow = obstaclePos / amountCols;
            int obstacleCol = obstaclePos % amountCols;

            bool isBlocked = maze[obstacleRow][obstacleCol] == '#';
            if (isBlocked)
            {
                continue;
            }
            maze[obstacleRow][obstacleCol] = '#'; // add obstacle
            
            currentPosition = intialPosition; // reset position
            currentDirection = 0; // reset direction to up

            amountOfLoops += IsMazeALoop(maze, currentPosition, currentDirection, amountRows, amountCols) ? 1 : 0;

            maze[obstacleRow][obstacleCol] = '.'; // reset maze
        }

        return amountOfLoops;
    }

    private static bool IsMazeALoop(char[][] maze, (int row, int col) currentPosition, int currentDirection, int amountRows, int amountCols)
    {
        Dictionary<(int row, int col), List<int>> visitedAs = new();
        CheckAndMarkVisited(visitedAs, currentPosition, currentDirection);
        while (true)
        {
            (int row, int col) nextPosition = AddTuples(currentPosition, directions[currentDirection]);

            bool isExit = nextPosition.row < 0 || nextPosition.row > amountRows ||
                          nextPosition.col < 0 || nextPosition.col > amountCols;
            if (isExit) return false;

            bool isBlocked = maze[nextPosition.row][nextPosition.col] == '#';
            if (isBlocked)
            {
                currentDirection = Turn(currentDirection);
            }
            else
            {
                currentPosition = nextPosition;
                if (CheckAndMarkVisited(visitedAs, currentPosition, currentDirection))
                    return true;
            }
        }
    }

    private static (int, int) FindStartingPosition(char[][] maze)
    {
        for (int row = 0; row < maze.Length; row++)
        {
            int col = Array.IndexOf(maze[row], '^');
            if (col != -1)
            {
                return (row, col);
            }
        }
        throw new Exception("Start not found");
    }

    public static bool CheckAndMarkVisited(Dictionary<(int row, int col), List<int>> visitedAs, (int row, int col) currentPosition, int currentDirection)
    {
        if (!visitedAs.TryGetValue(currentPosition, out var directions))
        {
            visitedAs[currentPosition] = new List<int> { currentDirection };
        }
        else if (directions.Contains(currentDirection))
        {
            return true;
        }
        else
        {
            directions.Add(currentDirection);
        }
        return false;
    }

    private static int Turn(int currentDirection) => (currentDirection + 1) % directions.Length;

    private static (int, int) AddTuples((int, int) tuple1, (int, int) tuple2) => (tuple1.Item1 + tuple2.Item1, tuple1.Item2 + tuple2.Item2);

}
