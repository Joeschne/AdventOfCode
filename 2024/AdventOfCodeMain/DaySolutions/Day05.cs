using System.Diagnostics;

namespace AdventOfCodeMain.DaySolutions;

internal class Day05
{
    public static void Run()
    {
        string[] challengeInput = FileReader.ReadFromFile("Day5.1").Split("\r\n\r\n");

        Dictionary<int, List<int>> rulesetDict = challengeInput[0]
            .Split(new string[] { "|", "\r\n" }, StringSplitOptions.None)
            .Select(int.Parse)
            .Chunk(2)
            .Select(chunk => new { Key = chunk[0], Value = chunk[1] }) // Create key-value pairs
            .GroupBy(x => x.Key) // Group by Key ([0])
            .ToDictionary(
                group => group.Key,
                group => group.Select(x => x.Value).ToList() // Group all [0] values into an array
            );

        int[][] updates = challengeInput[1]
            .Split("\r\n")
            .Select(update => update.Split(',').Select(int.Parse).ToArray())
            .ToArray();

        int correctMiddlePageNumbers = 0;
        int incorrectMiddlePageNumbers = 0;

        foreach (int[] update in updates)
        {
            bool validUpdate = true;
            int currentPage = 0;
            while (validUpdate && currentPage < update.Length)
            {
                validUpdate = ViolatesRuleset(rulesetDict, update, currentPage);
                currentPage++;
            }
            if (validUpdate)
            {
                correctMiddlePageNumbers += update[(update.Length - 1) / 2];
            }
            else
            {
                List<int> sortedUpdate = SortUpdate(rulesetDict, update);
                incorrectMiddlePageNumbers += sortedUpdate[(sortedUpdate.Count - 1) / 2];
            }
        }
        Console.WriteLine(correctMiddlePageNumbers);
        Console.WriteLine(incorrectMiddlePageNumbers);
    }

    private static bool ViolatesRuleset(Dictionary<int, List<int>> rulesetDict, int[] update, int currentPage)
    {
        if (rulesetDict.ContainsKey(update[currentPage]))
        {
            for (int i = 0; i < currentPage; i++)
            {
                if (rulesetDict[update[currentPage]].Contains(update[i]))
                    return false;
            }
        }
        return true;
    }
    private static List<int> SortUpdate(Dictionary<int, List<int>> rulesetDict, int[] update)
    {
        // build graph for all numbers in the update
        Dictionary<int, List<int>> graph = update.ToDictionary(page => page, page => new List<int>()); // this works nicely because each update has no page twice
        Dictionary<int, int> inDegree = graph.Keys.ToDictionary(page => page, page => 0);

        // add dependencies to graph based on ruleset
        foreach (int page in graph.Keys)
        {
            if (rulesetDict.TryGetValue(page, out List<int>? forbiddenPages))
            {
                foreach (int forbiddenPage in forbiddenPages)
                {
                    if (graph.ContainsKey(forbiddenPage))
                    {
                        graph[page].Add(forbiddenPage); 
                        inDegree[forbiddenPage]++; // increment in-degree for forbiddenPage (amount of dependencies)
                    }
                }
            }
        }

        return TopologicalSort(graph, inDegree);
    }

    private static List<int> TopologicalSort(Dictionary<int, List<int>> graph, Dictionary<int, int> inDegree)
    {
        Queue<int> queue = new Queue<int>(inDegree.Where(kvp => kvp.Value == 0).Select(kvp => kvp.Key));
        List<int> sortedList = new List<int>();

        while (queue.Count > 0)
        {
            int node = queue.Dequeue();
            sortedList.Add(node);

            foreach (int otherNode in graph[node])
            {
                if (--inDegree[otherNode] == 0) // node is a neighbor
                {
                    queue.Enqueue(otherNode);
                }
            }
        }

        if (sortedList.Count != graph.Count)
            throw new Exception("A cycle has been created. Check your input or you coded weirdly");
        return sortedList;
    }
}
