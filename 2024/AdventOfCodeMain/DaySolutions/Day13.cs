using System.Numerics;
using System.Text.RegularExpressions;

namespace AdventOfCodeMain.DaySolutions;

internal class Day13
{
    public static void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day13.1");
        
        string pattern = @"X\+(\d+), Y\+(\d+)|X=(\d+), Y=(\d+)";
        Regex regex = new Regex(pattern);

        List<(long, long)> aButtons = new();
        List<(long, long)> bButtons = new();
        List<(long x, long y)> prizes = new();

        var matches = regex.Matches(challengeInput);
        for (int i = 0; i < matches.Count; i += 3)
        {
            var matchA = matches[i];
            aButtons.Add((long.Parse(matchA.Groups[1].Value), long.Parse(matchA.Groups[2].Value)));

            var matchB = matches[i + 1];
            bButtons.Add((long.Parse(matchB.Groups[1].Value), long.Parse(matchB.Groups[2].Value)));

            var matchPrize = matches[i + 2];
            prizes.Add((long.Parse(matchPrize.Groups[3].Value), long.Parse(matchPrize.Groups[4].Value)));
        }

        // I just watched a bunch of math tutorials for this bcuz I don't remember anything, this shit better work
        long totalPrice = 0;
        long correctedTotalPrice = 0;
        for(int i = 0; i < prizes.Count; i++)
        {
            totalPrice += CalculatePrice(aButtons[i], bButtons[i], prizes[i]);
            correctedTotalPrice += CalculatePrice(aButtons[i], bButtons[i], (prizes[i].x + 10000000000000, prizes[i].y + 10000000000000));
        }

        Console.WriteLine($"Initial calculated price: {totalPrice}\nCorrected: {correctedTotalPrice}");
    }
    private static long CalculatePrice((long x, long y) aButton, (long x, long y) bButton, (long x, long y) prizeLocation)
    {
        // calculate determinant
        long determinant = bButton.x * aButton.y - bButton.y * aButton.x;

        // calculate bPresses and check divisibility
        long numerator = prizeLocation.x * aButton.y - prizeLocation.y * aButton.x;
        if (numerator % determinant != 0) return 0; // no integer solution
        long bPresses = numerator / determinant;

        // calculate aPresses and check divisibility
        long aNumerator = prizeLocation.x - bPresses * bButton.x;
        if (aNumerator % aButton.x != 0) return 0; // no integer solution
        long aPresses = aNumerator / aButton.x;

        return aPresses * 3 + bPresses;
    }
}
