using System.Collections.Generic;
using System.Diagnostics;
using System.Text;

namespace AdventOfCodeMain.DaySolutions;

internal class Day09
{
    public static void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day9.1");
        SolvePart1(challengeInput);
        SolvePart2(challengeInput);
    }
    private static void SolvePart1(string challengeInput)
    {
        List<string> fileMap = challengeInput
            .SelectMany((c, i) => Enumerable.Repeat(
                i % 2 == 0 ? (i / 2).ToString() : ".",
                int.Parse(c.ToString())
            ))
            .ToList();

        long checksum = 0;
        int rightIndex = fileMap.Count - 1;
        for (int i = 0; i < fileMap.Count; i++)
        {
            if (fileMap[i] != ".")
            {
                checksum += i * int.Parse(fileMap[i]);
                continue;
            }
            if (!fileMap.Skip(i).Any(n => int.TryParse(n, out _))) break;
            while (fileMap[rightIndex] == ".") rightIndex--;
            (fileMap[i], fileMap[rightIndex]) = (fileMap[rightIndex], fileMap[i]);
            checksum += i * int.Parse(fileMap[i]);
        }
        Console.WriteLine(checksum);
    }

    // this shit took me way too long but in the end it was kind of fun. shouldn't have been so difficult but I tried multiple other approaches I thought were cleaner, but they didn't really work out. way uglier than my usual code but who cares, I still need to catch up on 3 other days
    private static void SolvePart2(string challengeInput)
    {
        List<List<string>> fileMap = challengeInput
            .Select((c, i) => Enumerable.Repeat(
                i % 2 == 0 ? (i / 2).ToString() : ".",
                int.Parse(c.ToString())
            ).ToList())
            .ToList();

        for (int i = fileMap.Count - 1; i >= 0; i--)
        {
            if (fileMap[i].Count == 0 || !int.TryParse(fileMap[i][0], out _)) continue;
            for (int j = 0; j < i; j++)
            {
                if (fileMap[j].Count == 0 || fileMap[j][0] != ".") continue;  
                if (fileMap[j].Count == fileMap[i].Count)
                {
                    (fileMap[i], fileMap[j]) = (fileMap[j], fileMap[i]);      // switch packets
                    break;
                }
                else if (fileMap[j].Count > fileMap[i].Count)
                {
                    int difference = fileMap[j].Count - fileMap[i].Count;
                    int remainder = fileMap[j].Count - difference;
                    fileMap[j] = fileMap[i];                                  // move packet to nook
                    fileMap[i] = Enumerable.Repeat(".", remainder).ToList();
                    fileMap.Insert(j+1, Enumerable.Repeat(".", difference).ToList());
                    i++;
                    break;
                }
            }
        }

        // flattten and calculate checksum
        long checksum = fileMap
            .SelectMany(innerList => innerList)
            .Select((value, index) => value != "." ? (long)index * long.Parse(value) : 0)
            .Sum();

        Console.WriteLine(checksum);
    }
}
