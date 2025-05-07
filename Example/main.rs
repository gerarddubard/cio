// mains.rs
use cio::{println, input};
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet, VecDeque, LinkedList, BinaryHeap};

fn main() {
    // Program title
    println!("@(bright_cyan, bold)=== Demonstration of println! with std::collections and advanced formatting ===");

    /* SECTION 1: BASIC INPUT AND VARIABLE DISPLAY */
    println!("@(green, bold)1. Basic input and variable display:");
    let first_name: String = input!("Your @(green, italic)first name: ");
    let last_name: String = input!("Your @(red, bold)last name: ");
    let age: i32 = input!("Your @(yellow)age: ");
    let height: f64 = input!("Your @(blue)height@() (in meters): ");
    let married: bool = input!("Are you @(magenta)married@()? (true/false): ");
    let favorite_letter: char = input!("What is your @(cyan, italic)favorite letter@()? ");
    let status = if married { "you are" } else { "you are not" };
    println!("Hello, @(green, bold){first_name} @(red, bold){last_name}@(), you are @(yellow){age}@() years old, you are @(blue){height}@() m tall, your favorite letter is '@(magenta){favorite_letter}@()', and @(cyan, bold){status}@() married.");

    println!("\n@(bright_white, bold)------------------------------------------------\n");
    /* SECTION 2: EXPRESSIONS IN PLACEHOLDERS */
    println!("@(green, bold)2. Expressions in placeholders:");
    println!("Age in months: @(yellow){age * 12}");
    println!("Height in cm: @(blue){height * 100.0:.0}");
    println!("Last name in uppercase: @(red, bold){last_name.to_uppercase()}");

    // Using intermediate variables for complex expressions
    let first_letter = last_name.chars().next().unwrap_or('?');
    println!("First letter of the last name: @(magenta){first_letter}");
    println!("Is your favorite letter uppercase? @(cyan){favorite_letter.is_uppercase()}");
    println!("Your favorite letter ASCII value: @(yellow){favorite_letter as u8}");
    let letter_category = if favorite_letter.is_alphabetic() {
        if favorite_letter.is_ascii_lowercase() { "lowercase letter" }
        else { "uppercase letter" }
    } else if favorite_letter.is_numeric() {
        "digit"
    } else {
        "special character"
    };
    println!("Letter category: @(bright_green){letter_category}");
    let letter_position = favorite_letter.to_uppercase().to_string().chars().next().unwrap() as u8 - 64;
    println!("Letter analysis: @(bright_cyan){letter_position}@() (position in alphabet if letter)");

    println!("\n@(bright_white, bold)------------------------------------------------\n");
    /* SECTION 3: NUMBER FORMATTING */
    println!("@(green, bold)3. Number formatting:");
    let pi = std::f64::consts::PI;
    println!("@(yellow, italic)PI with various formats:");
    println!("  - Standard: {pi}");
    println!("  - With 2 decimals: {pi:.2}");
    println!("  - With 6 decimals: {pi:.6}");
    println!("  - Scientific notation: {pi:e}");
    println!("\n@(cyan, italic)Integer formats for {age}:");
    println!("  - Padded with zeros: {age:04}");
    println!("  - Hexadecimal lowercase: {age:x}");
    println!("  - Hexadecimal uppercase: {age:X}");
    println!("  - Binary: {age:b}");
    println!("  - Octal: {age:o}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");
    /* SECTION 4: SEPARATOR AND TERNARY OPERATORS WITH $(...) */
    println!("@(green, bold)4. Separator and ternary operators with $(...):");

    // Example 1: Test with a literal separator directly in $()
    println!("@(bright_yellow)Test with literal separator:");
    for i in 1..10 {
        println!("@(yellow){i}$( - )");
    }
    println!("@(yellow)10");

    let mut sep: &str;
    let mut color_style: &str;

    // Example 2: Direct use without intermediate variable to avoid problems
    println!("\n@(yellow, italic)Simple ternary with separator:");
    for i in 1..=10 {
        color_style = if i%2 == 0 {"bright_green"} else {"bright_red"};
        sep = if i < 10 {" → "} else {"\n"};
        println!("@(color_style){i}@()$(sep)");
    }
    println!("\n@(yellow, italic)Complex ternary with formatting:");

    // Example 3: Complex ternary operator with $()
    for i in 1..=5 {
        color_style = if i%3 == 0 {"bright_cyan, bold"} else if i%3 == 1 {"bright_magenta, italic"} else {"bright_yellow, underline"};
        let value = i * 5;
        let symbol = if value < 10 {"★"} else if value < 20 {"◆"} else {"▲"};
        sep = if i < 5 {" | "} else {"\n"};
        println!("@(color_style){symbol} {value}@()$(sep)");
    }

    println!("\n@(yellow, italic)Nested ternary expressions in separators:");
    // Example 4: Version without using $(...)
    sep = " ";
    let values = vec![12, 5, 8, 21, 3, 9, 15];
    for val in values {
        let color_code = if val < 10 { "@(green)" } else if val < 15 { "@(yellow)" } else { "@(red)" };
        let marker = if val % 2 == 0 { "▣" } else { "▢" };
        println!("@(color_code){marker}{val}$(sep)");
    }

    println!("\n\n@(yellow, italic)Dynamic collection formatting with separators:");
    // Example 5: Dynamic formatting of collections with match
    let temperatures = vec![22.5, 19.8, 25.3, 18.7, 24.9];
    println!("Temperatures: $()");
    // Find min and max values
    let max_temp = temperatures.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_temp = temperatures.iter().cloned().fold(f64::INFINITY, f64::min);

    for (idx, &temp) in temperatures.iter().enumerate() {
        // Determine style and color with match
        let color_style = match temp {
            t if t < 20.0 && (t - min_temp).abs() < 0.001 => "bright_blue, italic",
            t if t < 20.0 => "bright_blue",
            t if t < 25.0 => "bright_green",
            t if (t - max_temp).abs() < 0.001 => "bright_red, bold",
            _ => "bright_red"
        };

        // Add a separator if not the first element
        let sep = if idx < (temperatures.len() - 1) {" | "} else {"\n"};

        // Display the temperature with style and separator
        println!("@(color_style){temp:.1}°C$(sep)");
    }

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 5: MULTIDIMENSIONAL ARRAYS AND MATRICES */
    println!("@(green, bold)5. Multidimensional arrays and matrices:");

    println!("@(yellow)Array and matrix formatting demonstration:");

    // 1D array - Vector
    let vec_1d = vec![1, 2, 3, 4, 5];

    // 2D array - Square matrices
    let matrix_2x2 = vec![vec![4, 3], vec![2, 1]];
    let matrix_3x3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // 3D array
    let vec_3d = vec![
        vec![
            vec![1, 2],
            vec![3, 4]
        ],
        vec![
            vec![5, 6],
            vec![7, 8]
        ]
    ];

    // Format comparison for 1D arrays - simple vectors
    println!("@(blue, bold)1D Vector display formats:");
    println!("@(cyan)1D Vector (:a - array format):@()     {vec_1d:a}");
    println!("@(cyan)1D Vector (:c - compact format):@()   {vec_1d:c}");

    // Format comparison for 2D matrices with line break
    println!("\n@(blue, bold)2D Matrix display formats:");
    println!("@(yellow)2x2 Matrix (:a - array format):@() \n{matrix_2x2:a}");
    println!("\n@(yellow)2x2 Matrix (:c - compact format):@() {matrix_2x2:c}");
    println!("\n@(yellow)2x2 Matrix (:m - matrix format):@() \n{matrix_2x2:m}");
    println!("@(yellow)2x2 Matrix (:d - determinant format):@() \n{matrix_2x2:d}");

    println!("\n@(magenta)3x3 Matrix (:a - array format):@() \n{matrix_3x3:a}");
    println!("@(magenta)3x3 Matrix (:c - compact format):@() {matrix_3x3:c}");
    println!("\n@(magenta)3x3 Matrix (:m - matrix format):@() \n{matrix_3x3:m}");
    println!("@(magenta)3x3 Matrix (:d - determinant format):@() \n{matrix_3x3:d}");

    // Format comparison for 3D arrays
    println!("\n@(blue, bold)3D Array display formats:");
    println!("@(bright_cyan)3D Array (:a - array format):");
    println!("{vec_3d:a}");
    println!("\n@(bright_cyan)3D Array (:c - compact format):@() {vec_3d:c}");

    // Special matrix shapes - 2D with different dimensions
    println!("\n@(bright_magenta, bold)Special matrix shapes:");

    // Row matrix (1×N)
    let matrix_row = vec![vec![10, 20, 30, 40]];
    println!("@(bright_cyan)Row matrix (1x4) :");
    println!("  (:a) \n{matrix_row:a}");
    println!("\n  (:c) {matrix_row:c}");
    println!("\n  (:m) {matrix_row:m}");

    // Column matrix (N×1)
    let matrix_column = vec![vec![-1], vec![100], vec![3], vec![42]];
    println!("\n@(bright_cyan)Column matrix (4x1) :");
    println!("  (:c) \n{matrix_column:a}");
    println!("\n  (:a) {matrix_column:c}");
    println!("\n  (:m) \n{matrix_column:m}");

    // Rectangular matrix (N×M)
    let matrix_rect = vec![vec![11, 12, 13], vec![14, 15, 16]];
    println!("\n@(bright_cyan)Rectangular matrix (2x3) :");
    println!("  (:a) \n{matrix_rect:a}");
    println!("\n  (:c) {matrix_rect:c}");
    println!("\n  (:m) \n{matrix_rect:m}");

    // Other sequence containers
    println!("\n@(bright_green, bold)Other sequence containers:");

    let mut vecdeque = VecDeque::new();
    vecdeque.push_back(1);
    vecdeque.push_back(2);
    vecdeque.push_front(0);
    vecdeque.push_back(3);

    let mut linked_list = LinkedList::new();
    linked_list.push_back("first");
    linked_list.push_back("second");
    linked_list.push_back("third");

    let mut binary_heap = BinaryHeap::new();
    binary_heap.push(5);
    binary_heap.push(1);
    binary_heap.push(3);

    println!("@(bright_yellow)VecDeque:@()   {vecdeque:a}");
    println!("@(bright_yellow)LinkedList:@() {linked_list:a}");
    println!("@(bright_yellow)BinaryHeap:@() {binary_heap:a}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 6: ASSOCIATIVE CONTAINERS AND COMPLEX STRUCTURES */
    println!("@(green, bold)6. Associative containers and complex structures:");

    // Create different associative containers
    let mut hash_map = HashMap::new();
    hash_map.insert("France", "Paris");
    hash_map.insert("Germany", "Berlin");
    hash_map.insert("Italy", "Rome");

    let mut btree_map = BTreeMap::new();
    btree_map.insert("A", 1);
    btree_map.insert("C", 3);
    btree_map.insert("B", 2);

    let mut hash_set = HashSet::new();
    hash_set.insert("apple");
    hash_set.insert("banana");
    hash_set.insert("orange");

    let mut btree_set = BTreeSet::new();
    btree_set.insert(5);
    btree_set.insert(2);
    btree_set.insert(8);
    btree_set.insert(1);

    // Format comparison for associative containers - only :j and :c
    println!("@(bright_blue, bold)Display formats for associative containers:");

    println!("@(yellow)HashMap (:j - pretty format):@() \n{hash_map:j}");
    println!("\n@(yellow)HashMap (:c - compact format):@() {hash_map:c}");

    println!("\n@(yellow)BTreeMap (:j - pretty format):@() \n{btree_map:j}");
    println!("\n@(yellow)BTreeMap (:c - compact format):@() {btree_map:c}");

    println!("\n@(yellow)HashSet (:j - pretty format):@() \n{hash_set:j}");
    println!("\n@(yellow)HashSet (:c - compact format):@() {hash_set:c}");

    println!("\n@(yellow)BTreeSet (:j - pretty format):@() \n{btree_set:j}");
    println!("\n@(yellow)BTreeSet (:c - compact format):@() {btree_set:c}");

    // Operations on Maps
    println!("\n@(magenta, bold)Map operations:");
    let france_capital = hash_map.get("France").unwrap_or(&"Unknown");
    println!("Capital of France: @(bright_green){france_capital}");
    println!("Is 'Spain' in HashMap? @(bright_red){hash_map.contains_key(\"Spain\")}");

    // Complex nested structures
    println!("\n@(bright_magenta, bold)Complex nested structures:");

    // 3-level nested structure
    let mut cities_data = HashMap::new();

    let mut france = HashMap::new();
    let mut paris = HashMap::new();
    paris.insert("population".to_string(), "2.2M".to_string());
    paris.insert("attractions".to_string(), "Eiffel Tower, Louvre".to_string());
    france.insert("Paris".to_string(), paris);

    let mut usa = HashMap::new();
    let mut new_york = HashMap::new();
    new_york.insert("population".to_string(), "8.4M".to_string());
    new_york.insert("attractions".to_string(), "Statue of Liberty, Times Square".to_string());
    usa.insert("New York".to_string(), new_york);

    cities_data.insert("France".to_string(), france);
    cities_data.insert("USA".to_string(), usa);

    println!("@(blue)3-level nested structure (:j):@() \n{cities_data:j}");
    println!("\n@(blue)3-level nested structure (:c):@() \n{cities_data:c}");

    // Deep access to nested elements
    let unknown_population = "Unknown".to_string();
    let paris_population = cities_data
        .get("France")
        .and_then(|france| france.get("Paris"))
        .and_then(|paris| paris.get("population"))
        .unwrap_or(&unknown_population);

    println!("\nPopulation of Paris: @(green){paris_population}");

    let no_attractions = "No attractions listed".to_string();
    let attractions_ny = cities_data
        .get("USA")
        .and_then(|country| country.get("New York"))
        .and_then(|city| city.get("attractions"))
        .unwrap_or(&no_attractions);

    println!("Attractions in New York: @(cyan){attractions_ny}");
    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 7: DATA ANALYSIS WITH TURBOFISH */
    println!("@(green, bold)7. Data analysis with turbofish:");

    // Create sample dataset for analysis
    let dataset = vec![
        ("Alice", 28, 75.5, "Marketing"),
        ("Bob", 35, 82.3, "Engineering"),
        ("Charlie", 42, 91.0, "Management"),
        ("Diana", 31, 65.8, "Marketing"),
        ("Eva", 27, 68.2, "Engineering"),
        ("Frank", 44, 88.7, "Finance"),
        ("Grace", 38, 72.1, "Management"),
        ("Henry", 29, 79.4, "Engineering"),
        ("Ivy", 33, 63.5, "Marketing"),
        ("Jack", 41, 84.9, "Finance"),
    ];

    println!("@(yellow, italic)Dataset for analysis:");
    println!("@(bright_cyan)Name      Age     Weight    Department");
    for (name, age, weight, dept) in &dataset {
        // Using fixed width formatting for alignment
        println!("{name:<10}{age:<8}{weight:<10.1}{dept}");
    }

    // Example 1: Basic statistics - count by department
    let mut dept_count = HashMap::new();
    for (_, _, _, dept) in &dataset {
        *dept_count.entry(dept).or_insert(0) += 1;
    }

    println!("\n@(blue, bold)Department counts:");
    let mut dept_stats = Vec::new();
    for (dept, count) in &dept_count {
        let percentage = (*count as f64 / dataset.len() as f64) * 100.0;
        dept_stats.push(format!("{}: {} ({:.1}%)", dept, count, percentage));
    }
    println!("{dept_stats.join(\", \")}");

    // Example 2: Advanced grouping and aggregation
    println!("\n@(blue, bold)Department statistics:");

    // Group by department
    let mut dept_data: HashMap<&str, Vec<(i32, f64)>> = HashMap::new();
    for (_, age, weight, dept) in &dataset {
        dept_data.entry(dept).or_insert_with(Vec::new).push((*age, *weight));
    }

    // Calculate statistics per department
    let mut dept_stats = Vec::new();
    for (dept, values) in &dept_data {
        let count = values.len();

        // Calculate averages
        let avg_age = values.iter().map(|(age, _)| age).sum::<i32>() as f64 / count as f64;
        let avg_weight = values.iter().map(|(_, weight)| weight).sum::<f64>() / count as f64;

        // Find min/max
        let min_age = values.iter().map(|(age, _)| age).min().unwrap_or(&0);
        let max_age = values.iter().map(|(age, _)| age).max().unwrap_or(&0);

        // Add to stats
        dept_stats.push((*dept, count, avg_age, avg_weight, *min_age, *max_age));
    }

    // Sort by average age and display
    dept_stats.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    println!("@(cyan)Department    Count    Avg Age    Avg Weight    Age Range");
    for (dept, count, avg_age, avg_weight, min_age, max_age) in dept_stats {
        println!("{dept:<12}  {count}        {avg_age:.1}       {avg_weight:.1}         {min_age}-{max_age}");
    }

    // Example 3: Data transformation and functional approach
    println!("\n@(blue, bold)Data transformation:");

    // Using iterators and functional approach
    let age_categories = dataset.iter()
        .map(|(name, age, _, dept)| {
            let category = if *age < 30 {
                "Young"
            } else if *age < 40 {
                "Mid-Career"
            } else {
                "Senior"
            };
            (category, name, dept)
        })
        .collect::<Vec<_>>();

    // Group by age category
    let mut category_data: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    for (category, name, dept) in age_categories {
        category_data.entry(category).or_insert_with(Vec::new).push((name, dept));
    }

    // Display data by category
    println!("@(cyan)Age distribution by category:");
    let categories = ["Young", "Mid-Career", "Senior"];
    for category in categories {
        if let Some(people) = category_data.get(category) {
            let count = people.len();
            let percentage = (count as f64 / dataset.len() as f64) * 100.0;

            let names = people.iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
                .join(", ");

            println!("@(yellow){category}@(): {count} people ({percentage:.1}%) - {names}");
        }
    }

    // Example 4: Custom types and self implementation
    println!("\n@(blue, bold)Using custom types with turbofish:");

    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
        weight: f64,
        department: String,
    }

    impl Person {
        fn new(name: &str, age: i32, weight: f64, department: &str) -> Self {
            Person {
                name: name.to_string(),
                age,
                department: department.to_string(),
                weight,
            }
        }

        fn bmi(&self) -> f64 {
            // Assuming height is fixed at 1.75m for simplicity
            let height = 1.75;
            self.weight / (height * height)
        }

        fn description(&self) {
            let (color, category) = if self.bmi() < 18.5 {
                ("blue", "underweight")
            } else if self.bmi() < 25.0 {
                ("green", "normal weight")
            } else if self.bmi() < 30.0 {
                ("yellow", "overweight")
            } else {
                ("red", "obese")
            };

            println!("@(bright_green)Hello, I am @(bright_yellow){self.name}. I am @(bright_cyan){self.age} years old, work in @(bright_magenta){self.department} and my BMI is {self.bmi():.1} @({color}){category}");
        }
    }

    // Convert dataset to Person objects
    let people: Vec<Person> = dataset.iter()
        .map(|(name, age, weight, dept)| {
            Person::new(name, *age, *weight, dept)
        })
        .collect();

    // Example of using self.method() with custom types
    println!("@(bright_blue, bold)Person descriptions using self methods:");
    people[0].description();
    people[2].description();

    // Example 5: Advanced data analysis
    println!("\n@(blue, bold)Advanced data analysis:");

    // Calculate stats per age group
    let avg_weight_by_age_group: HashMap<&str, f64> = people.iter()
        .map(|p| {
            let age_group = if p.age < 30 {
                "Young"
            } else if p.age < 40 {
                "Mid-Career"
            } else {
                "Senior"
            };
            (age_group, p.weight)
        })
        .fold(HashMap::<&str, (f64, i32)>::new(), |mut acc, (group, weight)| {
            let entry = acc.entry(group).or_insert((0.0, 0));
            entry.0 += weight;
            entry.1 += 1;
            acc
        })
        .iter()
        .map(|(group, (sum, count))| (*group, sum / *count as f64))
        .collect();

    println!("@(cyan)Average weight by age group:");
    for (group, avg) in &avg_weight_by_age_group {
        println!("{group}: {avg:.1} kg");
    }

    // Calculate cross-tabulation
    println!("\n@(cyan)Average age by department and weight category:");

    // First, categorize people by weight
    let weight_categories = ["Light (<70kg)", "Medium (70-80kg)", "Heavy (>80kg)"];

    // Create a matrix of department x weight category
    let mut dept_weight_matrix: HashMap<&str, HashMap<&str, (i32, i32)>> = HashMap::new();

    for p in &people {
        let weight_cat = if p.weight < 70.0 {
            "Light (<70kg)"
        } else if p.weight <= 80.0 {
            "Medium (70-80kg)"
        } else {
            "Heavy (>80kg)"
        };

        let dept_entry = dept_weight_matrix.entry(&p.department).or_insert_with(HashMap::new);
        let count_entry = dept_entry.entry(weight_cat).or_insert((0, 0));
        count_entry.0 += p.age;
        count_entry.1 += 1;
    }

    // Print table header - fixed version
    println!("Department    Light (<70kg)     Medium (70-80kg)  Heavy (>80kg)");

    // Print data rows
    let departments = ["Engineering", "Finance", "Management", "Marketing"];
    for dept in &departments {
        let mut row = format!("{:12}", dept);

        for cat in &weight_categories {
            if let Some(dept_map) = dept_weight_matrix.get(dept) {
                if let Some((age_sum, count)) = dept_map.get(cat) {
                    if *count > 0 {
                        let avg = *age_sum as f64 / *count as f64;
                        row.push_str(&format!(" {:7.1} ({:1})   ", avg, count));
                    } else {
                        row.push_str("     -         ");
                    }
                } else {
                    row.push_str("     -         ");
                }
            } else {
                row.push_str("     -         ");
            }
        }

        println!("{row}");
    }

    println!("\n@(bright_white, bold)------------------------------------------------\n");
    /* SECTION 8: FORMAT RECOMMENDATION SUMMARY */
    println!("@(green, bold)8. Format recommendation summary:");
    println!("- Format @(bright_cyan):a@(): Best for arrays and general data structures (adds indentation)");
    println!("- Format @(bright_magenta):j@(): Best for maps and complex structures (pretty-printed)");
    println!("- Format @(bright_yellow):c@(): Best for compact display (single-line for complex structures)");
    println!("- Format @(bright_green):m@(): Best for matrices (with special matrix border characters)");
    println!("- Format @(bright_red):d@(): Best for displaying matrix determinants (with vertical bars)");

    // Conclusion
    println!("\n@(bright_cyan, bold, italic)=== End of the demonstration ===");
}