namespace AdventOfCodeMain.DaySolutions;

internal class Day01
{
    public static void Run()
    {
        string challengeInput = FileReader.ReadFromFile("Day1.1");
        string[] splitInput = challengeInput.Split(new[] { " ", "\r\n" }, StringSplitOptions.RemoveEmptyEntries);
        var leftList = splitInput.Where((_, i) => i % 2 == 0).Select(int.Parse).ToArray();
        var rightList = splitInput.Where((_, i) => i % 2 != 0).Select(int.Parse).ToArray();

        Array.Sort(leftList);
        Array.Sort(rightList);

        long distanceResult = 0;
        long similarityScore = 0;

        for (int i = 0; i < leftList.Length; i++)
        {
            // first task
            distanceResult += Math.Abs(leftList[i] - rightList[i]);

            // second task
            int similarCounter = 0;
            int index = FirstOccurrence(rightList, leftList[i]);
            if (index >= 0)
            {
                while (index < rightList.Length && rightList[index] == leftList[i])
                {
                    similarCounter++;
                    index++;
                }
            }
            similarityScore += leftList[i] * similarCounter;
        }

        Console.WriteLine($"Distance: {distanceResult}\nSimilarity Score: {similarityScore}");
    }

    static int FirstOccurrence(int[] array, int target)
    {
        int left = 0;
        int right = array.Length - 1;
        int result = -1;

        while (left <= right)
        {
            int mid = left + (right - left) / 2;

            if (array[mid] == target)
            {
                result = mid;
                right = mid - 1; // Continue searching to the left
            }
            else if (array[mid] < target)
            {
                left = mid + 1;
            }
            else
            {
                right = mid - 1;
            }
        }

        return result;
    }
}