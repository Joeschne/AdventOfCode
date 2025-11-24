using System.Text.RegularExpressions;

namespace AdventOfCodeMain.DaySolutions;

internal class Day03
{
    static public void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day3.1");

        Console.WriteLine($"Total sum: {CheckPart1(challengeInput)}");
        Console.WriteLine($"Total sum conditional: {CheckPart2(challengeInput)}");

    }
    static private int CheckPart1(string challengeInput)
    {
        string pattern = @"mul\((\d{1,3}),(\d{1,3})\)";
        Regex multiplicationRegex = new Regex(pattern);

        int totalSum = 0;

        MatchCollection matches = multiplicationRegex.Matches(challengeInput);

        foreach (Match match in matches)
        {
            int num1 = int.Parse(match.Groups[1].Value);
            int num2 = int.Parse(match.Groups[2].Value);

            totalSum += num1 * num2;
        }
        return totalSum;
    }
    static private int CheckPart2(string challengeInput)
    {
        int totalSum = 0;

        string conditionalsPattern = @"don't\(\).*?(do\(\)|$)";

        string result = Regex.Replace(challengeInput, conditionalsPattern, "", RegexOptions.Singleline);

        return CheckPart1(result);
    }
}
