namespace AdventOfCodeMain.DaySolutions;

internal class Day02
{
    public static void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day2.1");
        string[] reports = challengeInput.Split("\r\n");
        int safeReports = 0;
        int bufferedSafeReports = 0;
        foreach (string report in reports)
        {
            List<int> levels = report.Split(" ").Select(int.Parse).ToList();
            if (CheckPart1(levels))
            {
                safeReports++;
                bufferedSafeReports++;
            }
            else if (CheckPart2(levels))
                bufferedSafeReports++;
        }
        Console.WriteLine(safeReports);
        Console.WriteLine(bufferedSafeReports);
    }
    private static bool CheckPart1(List<int> levels)
    {
        bool ascendingTrue = true;
        bool descendingTrue = true;
        int i = 0;
        while ((ascendingTrue || descendingTrue) && i < levels.Count - 1)
        {
            int difference = levels[i] - levels[i + 1];

            // Check for descending condition
            if (descendingTrue && (difference < 1 || difference > 3))
            {
                descendingTrue = false;
            }

            // Check for ascending condition
            if (ascendingTrue && (difference > -1 || difference < -3))
            {
                ascendingTrue = false;
            }

            i++;
        }
        if (ascendingTrue || descendingTrue)
        {
            return true;
        }
        return false;
    }
    private static bool CheckPart2(List<int> levels)
    {
        // fuck this shit I'm brute forcing it 
        for (int i = 0; i < levels.Count; i++)
        {
            var copy = new List<int>(levels);
            copy.RemoveAt(i);
            if (CheckPart1(copy))
            {
                return true;
            }
        }
        return false;
    }

}
