using System.Diagnostics;

using System.Diagnostics;

namespace AdventOfCodeMain.DaySolutions;

internal class Day11
{
    public static void Run()
    {
        string[] challengeInput = FileReader.ReadFromFile("Day11.1").Split(" ").ToArray();

        for (int i = 0; i < 25; i++)
        {
            Console.WriteLine(i);
            challengeInput = Blink(challengeInput);
        }

        Console.WriteLine(challengeInput.Length);

    }
    private static string[] Blink(string[] input)
    {
        // count how many items we'll add
        // odd-length strings add 1 item, even-length strings add 2 items, and zero-starting strings add 1.
        int newCount = 0;
        for (int i = 0; i < input.Length; i++)
        {
            var length = input[i].Length;
            if ((length & 1) == 0) // => even
            {
                newCount += 2;
            }
            else
            {
                newCount += 1;
            }
        }

        var newAlignment = new string[newCount];
        int index = 0;

        for (int i = 0; i < input.Length; i++)
        {
            var stoneString = input[i];
            var length = stoneString.Length;
            char firstChar = stoneString[0];

            if (firstChar == '0') newAlignment[index++] = "1";
            else if ((length & 1) == 0)
            {
                int mid = length >> 1; // length/2 using bit shift
                string firstHalf = stoneString[..mid];

                // truncate leading zeros in second half
                while (mid < length && stoneString[mid] == '0') mid++;
                string secondHalf = (mid == length) ? "0" : stoneString[mid..];

                newAlignment[index++] = firstHalf;
                newAlignment[index++] = secondHalf;
            }
            else
            {
                newAlignment[index++] = (long.Parse(stoneString) * 2024).ToString();
            }
        }

        return newAlignment;
    }

}
