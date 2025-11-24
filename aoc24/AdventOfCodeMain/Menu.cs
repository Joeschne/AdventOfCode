using System;

namespace AdventOfCodeMain;

public class Menu
{
    public static void ShowMenu()
    {
        Console.WriteLine("Welcome to Advent of Code 2024!");
        Console.WriteLine("Select a day to run (1-25) or press 0 to exit:");
        Console.WriteLine("---------------------------------------------");

        while (true)
        {
            Console.Write("Enter your choice: ");
            string input = Console.ReadLine();

            if (int.TryParse(input, out int day))
            {
                if (day == 0)
                {
                    Console.WriteLine("Exiting Advent of Code 2024. Goodbye!");
                    break;
                }
                else if (day >= 1 && day <= 25)
                {
                    RunDay(day);
                }
                else
                {
                    Console.WriteLine("Invalid choice. Please enter a number between 1 and 25, or 0 to exit.");
                }
            }
            else
            {
                Console.WriteLine("Invalid input. Please enter a valid number.");
            }
        }
    }

    private static void RunDay(int day)
    {
        Console.WriteLine($"Running solution for Day {day}...");
        
        // Dynamically call the day's solution class
        string dayClassName = $"AdventOfCodeMain.DaySolutions.Day{day:D2}";
        Type type = Type.GetType(dayClassName);

        if (type != null)
        {
            var method = type.GetMethod("Run");
            if (method != null)
            {
                method.Invoke(null, null);
            }
            else
            {
                Console.WriteLine($"No 'Run' method found for Day {day}. Please implement it.");
            }
        }
        else
        {
            Console.WriteLine($"Solution for Day {day} is not yet implemented.");
        }
        
    }
}
