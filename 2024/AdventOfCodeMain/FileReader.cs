using System;
using System.IO;

namespace AdventOfCodeMain;

public static class FileReader
{
    /// <summary>
    /// Reads the content of a file from the InputFiles directory.
    /// </summary>
    /// <param name="filename">The name of the file to read (e.g., "Day1.1.txt").</param>
    /// <returns>The content of the file as a string.</returns>
    public static string ReadFromFile(string filename)
    {
        try
        {
            // Construct the full path to the file
            string filePath = Path.Combine("../../../../", "InputFiles", $"{filename}.txt");

            filePath = Path.GetFullPath(filePath);

            // Check if the file exists
            if (!File.Exists(filePath))
            {
                throw new FileNotFoundException($"File not found: {filePath}");
            }

            // Read and return the file content
            return File.ReadAllText(filePath);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error reading file: {ex.Message}");
            return string.Empty;
        }
    }
}

