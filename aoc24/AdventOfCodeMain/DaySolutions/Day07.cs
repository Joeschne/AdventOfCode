namespace AdventOfCodeMain.DaySolutions;

internal class Day07
{
    public static void Run()
    {
        string[] challengeInput = FileReader.ReadFromFile("Day7.1")
            .Split(new string[] { ":", "\r\n" }, StringSplitOptions.None);

        long[] testValues = challengeInput
            .Where((_, index) => index % 2 == 0)
            .Select(long.Parse)
            .ToArray();

        int[][] calibrations = challengeInput
            .Where((_, index) => index % 2 != 0) 
            .Select(equationNumbers =>
                equationNumbers.Split(' ', StringSplitOptions.RemoveEmptyEntries)
                    .Select(part => int.Parse(part)) 
                    .ToArray()
            )
            .ToArray();


        (long totalCalibration, long totalCalibration2) = FindViableCalibrations(testValues, calibrations);

        Console.WriteLine($"Part 1: {totalCalibration}\nPart 2: {totalCalibration2}");
    }

    private static (long, long) FindViableCalibrations(long[] testValues, int[][] calibrations)
    {
        long totalCalibration = 0;
        long totalCalibration2 = 0;

        for (int i = 0; i < testValues.Length; i++)
        {
            if (IsViableCalculation(testValues[i], calibrations[i], false)) // coincatenation disallowed
            {
                totalCalibration += testValues[i];
                totalCalibration2 += testValues[i];
            }
            else if (IsViableCalculation(testValues[i], calibrations[i], true)) // concatenation allowed
            {
                totalCalibration2 += testValues[i];
            }
        }

        return (totalCalibration, totalCalibration2);
    }

    private static bool IsViableCalculation(long testValue, int[] calibration, bool allowConcatenation, int currentIndex = 0, long currentValue = 0)
    {
        if (currentValue > testValue)
            return false;

        bool endOfCalibrationReached = currentIndex == calibration.Length;
        if (endOfCalibrationReached)
        {
            return currentValue == testValue;
        }

        long num = calibration[currentIndex];

        // try adding
        if (IsViableCalculation(testValue, calibration, allowConcatenation, currentIndex + 1, currentValue + num))
        {
            return true;
        }

        // try multiplying
        if (IsViableCalculation(testValue, calibration, allowConcatenation, currentIndex + 1, currentValue * num))
        {
            return true;
        }

        // try concatenation if allowed
        if (allowConcatenation && IsViableCalculation(testValue, calibration, allowConcatenation, currentIndex + 1, long.Parse(currentValue.ToString() + num.ToString())))
        {
            return true;
        }

        return false;
    }

}
