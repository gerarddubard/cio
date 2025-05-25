use cio::{println, input};
use serde_json::json;
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet, VecDeque};

fn main() {
    println!("@(bright_cyan, bold)=== PyRust Demonstration: JSON + Native Collections + Advanced Formatting ===");

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
    let first_letter = last_name.chars().next().unwrap_or('?');
    println!("First letter of the last name: @(magenta){first_letter}");
    println!("Is your favorite letter uppercase? @(cyan){favorite_letter.is_uppercase()}");
    let letter_category = if favorite_letter.is_alphabetic() {
        if favorite_letter.is_ascii_lowercase() { "lowercase letter" }
        else { "uppercase letter" }
    } else if favorite_letter.is_numeric() {
        "digit"
    } else {
        "special character"
    };
    println!("Letter category: @(bright_green){letter_category}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 3: DYNAMIC SEPARATORS WITH $(...) */
    println!("@(green, bold)3. Dynamic separators with $(...):");
    println!("@(bright_yellow)Progress indicators:");
    for i in 1..10 {
        println!("@(yellow){i}$( → )");
    }
    println!("@(yellow)10");

    println!("\n@(yellow, italic)Temperature analysis:");
    let temperatures = vec![22.5, 19.8, 25.3, 18.7, 24.9];
    let max_temp = temperatures.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_temp = temperatures.iter().cloned().fold(f64::INFINITY, f64::min);
    for (idx, &temp) in temperatures.iter().enumerate() {
        let color_style = match temp {
            t if t < 20.0 && (t - min_temp).abs() < 0.001 => "bright_blue, italic",
            t if t < 20.0 => "bright_blue",
            t if t < 25.0 => "bright_green",
            t if (t - max_temp).abs() < 0.001 => "bright_red, bold",
            _ => "bright_red"
        };
        let sep = if idx < (temperatures.len() - 1) {" | "} else {"\n"};
        println!("@(color_style){temp:.1}°C$(sep)");
    }

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 4: JSON MATRICES AND ADVANCED TABLE FORMATTING */
    println!("@(green, bold)4. JSON matrices and advanced table formatting:");

    // Using json! for clean matrix declarations
    let matrix_1d = json!([1, 2, 3, 4, 5]);
    let matrix_2d = json!([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]);
    let matrix_3d = json!([
        [[1, 2], [3, 4]],
        [[5, 6], [7, 8]]
    ]);

    println!("@(cyan, bold)1D Array (JSON):@() \n{matrix_1d:t}");
    println!("\n@(green, bold)2D Matrix (JSON):@() \n{matrix_2d:t}");
    println!("\n@(yellow, bold)3D Matrix (JSON):@() \n{matrix_3d:t}");

    // Mixed approach: Native vectors for mathematical operations
    let native_matrix_2d = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let native_matrix_small = vec![vec![4, 3], vec![2, 1]];

    println!("\n@(blue, bold)Native matrices for mathematical formatting:");
    println!("@(yellow)3x3 Matrix (:m - mathematical format):@() \n{native_matrix_2d:m}");
    println!("@(yellow)3x3 Matrix (:d - determinant format):@() \n{native_matrix_2d:d}");
    println!("@(cyan)2x2 Matrix (:m - mathematical format):@() \n{native_matrix_small:m}");
    println!("@(cyan)2x2 Matrix (:d - determinant format):@() \n{native_matrix_small:d}");

    let sum: i32 = native_matrix_small.iter().flatten().sum();
    println!("\n@(magenta, bold)Native matrix operations:");
    println!("Matrix: \n{native_matrix_small:m}");
    println!("Sum of all elements: @(bright_green){sum}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 5: MIXED JSON AND NATIVE COLLECTIONS */
    println!("@(green, bold)5. Mixed JSON and native collections:");

    // JSON for structured data display
    let capitals = json!({
        "France": "Paris",
        "Italy": "Rome",
        "Germany": "Berlin",
        "Spain": "Madrid"
    });
    println!("@(blue, bold)Capitals (JSON format):@() \n{capitals:t(Country, Capital)}");

    // Native HashMap for operations and analysis
    let mut native_capitals = HashMap::new();
    native_capitals.insert("France", "Paris");
    native_capitals.insert("Italy", "Rome");
    native_capitals.insert("Germany", "Berlin");
    native_capitals.insert("Spain", "Madrid");

    println!("\n@(cyan, bold)Native HashMap operations:");
    let france_capital = native_capitals.get("France").unwrap_or(&"Unknown");
    let has_spain = native_capitals.contains_key("Spain");
    let total_countries = native_capitals.len();
    println!("Capital of France: @(bright_green){france_capital}");
    println!("Contains Spain? @(bright_yellow){has_spain}");
    println!("Total countries: @(bright_magenta){total_countries}");
    println!("Native HashMap display: \n{native_capitals:t(Country, Capital)}");

    // Show different std::collections with table formatting
    println!("\n@(bright_green, bold)Native std::collections showcase:");

    let mut vecdeque = VecDeque::new();
    vecdeque.push_back("First");
    vecdeque.push_back("Second");
    vecdeque.push_front("Zero");
    vecdeque.push_back("Third");

    let mut btree_map = BTreeMap::new();
    btree_map.insert("C", 3);
    btree_map.insert("A", 1);
    btree_map.insert("B", 2);

    let mut hash_set = HashSet::new();
    hash_set.insert("apple");
    hash_set.insert("banana");
    hash_set.insert("cherry");

    println!("@(yellow)VecDeque:@() \n{vecdeque:t}");
    println!("@(yellow)BTreeMap:@() \n{btree_map:t(Key, Value)}");
    println!("@(yellow)HashSet:@() \n{hash_set:t}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 6: COMPLEX JSON STRUCTURES */
    println!("@(green, bold)6. Complex JSON structures:");

    let class_notes_3d = json!({
        "6A": {
            "Français": [16.0, 13.0, 18.0, 15.0, 17.0],
            "Mathématiques": [15.0, 11.0, 16.0, 14.0, 15.0]
        },
        "6B": {
            "Français": [14.0, 12.0, 15.0, 13.0, 14.0],
            "Mathématiques": [12.0, 11.0, 14.0, 13.0, 11.0]
        },
        "6C": {
            "Français": [17.0, 14.0, 19.0, 16.0, 18.0],
            "Mathématiques": [16.0, 13.0, 17.0, 15.0, 16.0]
        }
    });

    let cities_data = json!({
        "France": {
            "Paris": {
                "population": "2.2M",
                "attractions": "Eiffel Tower"
            },
            "Lyon": {
                "population": "0.5M",
                "attractions": "Basilique de Fourvière"
            }
        },
        "USA": {
            "New York": {
                "population": "8.4M",
                "attractions": "Statue of Liberty"
            },
            "Los Angeles": {
                "population": "3.8M",
                "attractions": "Hollywood"
            }
        }
    });

    println!("@(red, bold)Class grades (3D JSON):@() \n{class_notes_3d:t}");
    println!("@(bright_blue, bold)Cities data (3D JSON):@() \n{cities_data:t}");

    // Matrix with explicit labels using JSON
    let matrix_with_labels = json!([
        {"": "Row A", "x": 1, "y": 2, "z": 3},
        {"": "Row B", "x": 4, "y": 5, "z": 6},
        {"": "Row C", "x": 7, "y": 8, "z": 9}
    ]);
    println!("\n@(magenta, bold)Matrix with labels (JSON):@() \n{matrix_with_labels:t}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 7: DATA ANALYSIS WITH NATIVE COLLECTIONS */
    println!("@(green, bold)7. Data analysis with native collections:");

    // Native collections for real data analysis
    let dataset = vec![
        ("Alice", 28, 75.5, "Marketing"),
        ("Bob", 35, 82.3, "Engineering"),
        ("Charlie", 42, 91.0, "Management"),
        ("Diana", 31, 65.8, "Marketing"),
        ("Eva", 27, 68.2, "Engineering"),
        ("Frank", 44, 88.7, "Finance"),
        ("Grace", 38, 72.1, "Management"),
        ("Henry", 29, 79.4, "Engineering"),
    ];

    println!("@(yellow, italic)Employee dataset analysis:");

    // Department statistics with turbofish examples
    let mut dept_count = HashMap::<&str, i32>::new();
    let mut dept_ages = HashMap::<&str, Vec<i32>>::new();
    let mut dept_weights = HashMap::<&str, Vec<f64>>::new();

    for (_name, age, weight, dept) in &dataset {
        *dept_count.entry(*dept).or_insert(0) += 1;
        dept_ages.entry(*dept).or_insert_with(Vec::<i32>::new).push(*age);
        dept_weights.entry(*dept).or_insert_with(Vec::<f64>::new).push(*weight);
    }

    println!("\n@(blue, bold)Department statistics with turbofish:");
    for (dept, count) in &dept_count {
        let ages = &dept_ages[dept];
        let weights = &dept_weights[dept];
        let avg_age = ages.iter().sum::<i32>() as f64 / ages.len() as f64;
        let avg_weight = weights.iter().sum::<f64>() / weights.len() as f64;
        let percentage = (*count as f64 / dataset.len() as f64) * 100.0;
        println!("@(cyan){dept}@(): {count} people ({percentage:.1}%), avg age: {avg_age:.1}, avg weight: {avg_weight:.1}kg");
    }

    // Advanced functional programming with turbofish
    println!("\n@(green, bold)Advanced turbofish examples:");

    // Example 1: Complex collect with type annotation
    let department_summary: Vec<(String, usize, f64)> = dept_count.iter()
        .map(|(dept, count)| {
            let avg_age = dept_ages[dept].iter().sum::<i32>() as f64 / dept_ages[dept].len() as f64;
            (dept.to_string(), *count as usize, avg_age)
        })
        .collect::<Vec<(String, usize, f64)>>();

    println!("@(yellow)Department summary (Vec<(String, usize, f64)>):@()");
    for (dept, count, avg_age) in &department_summary {
        println!("  {dept}: {count} people, avg age {avg_age:.1}");
    }

    // Example 2: BTreeMap with turbofish for sorted output
    let sorted_departments: BTreeMap<&str, (i32, f64)> = dataset.iter()
        .fold(BTreeMap::<&str, (i32, f64, i32)>::new(), |mut acc, (_name, age, weight, dept)| {
            let entry = acc.entry(*dept).or_insert((0, 0.0, 0));
            entry.0 += *age;
            entry.1 += *weight;
            entry.2 += 1;
            acc
        })
        .into_iter()
        .map(|(dept, (age_sum, weight_sum, count))| {
            (dept, (age_sum / count, weight_sum / count as f64))
        })
        .collect::<BTreeMap<&str, (i32, f64)>>();

    println!("\n@(yellow)Sorted departments (BTreeMap<&str, (i32, f64)>):@()");
    for (dept, (avg_age, avg_weight)) in &sorted_departments {
        println!("  {dept}: avg age {avg_age}, avg weight {avg_weight:.1}kg");
    }

    // Example 3: HashSet operations with turbofish
    let engineering_people: HashSet<&str> = dataset.iter()
        .filter(|(_name, _age, _weight, dept)| *dept == "Engineering")
        .map(|(name, _age, _weight, _dept)| *name)
        .collect::<HashSet<&str>>();

    let senior_people: HashSet<&str> = dataset.iter()
        .filter(|(_name, age, _weight, _dept)| *age >= 35)
        .map(|(name, _age, _weight, _dept)| *name)
        .collect::<HashSet<&str>>();

    println!("\n@(yellow)Set operations with turbofish:");
    let engineers_list = engineering_people.iter().cloned().collect::<Vec<&str>>().join(", ");
    let senior_list = senior_people.iter().cloned().collect::<Vec<&str>>().join(", ");
    let senior_engineers_list = engineering_people.intersection(&senior_people).cloned().collect::<Vec<&str>>().join(", ");
    println!("Engineers: {engineers_list}");
    println!("Senior people (35+): {senior_list}");
    println!("Senior engineers: {senior_engineers_list}");

    // Age categorization with explicit type annotations
    let age_distribution: HashMap<&str, Vec<(&str, &str)>> = dataset.iter()
        .map(|(name, age, _weight, dept)| {
            let category = if *age < 30 { "Young" } else if *age < 40 { "Mid-Career" } else { "Senior" };
            (category, (*name, *dept))
        })
        .fold(HashMap::<&str, Vec<(&str, &str)>>::new(), |mut acc, (cat, person)| {
            acc.entry(cat).or_insert_with(Vec::<(&str, &str)>::new).push(person);
            acc
        });

    println!("\n@(blue, bold)Age distribution with explicit types:");
    for (category, people) in &age_distribution {
        let names: Vec<&str> = people.iter().map(|(name, _dept)| *name).collect::<Vec<&str>>();
        let people_count = people.len();
        let names_list = names.join(", ");
        println!("@(yellow){category}@(): {people_count} people - {names_list}");
    }

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 8: ADVANCED TURBOFISH AND DATA SCIENCE */
    println!("@(green, bold)8. Advanced turbofish and data science:");

    // Create a more complex dataset for advanced analysis
    let sales_data = vec![
        ("Q1", "North", "Software", 12000.0, 150),
        ("Q1", "South", "Software", 8500.0, 110),
        ("Q1", "North", "Hardware", 15000.0, 75),
        ("Q1", "South", "Hardware", 9800.0, 65),
        ("Q2", "North", "Software", 14500.0, 180),
        ("Q2", "South", "Software", 11200.0, 140),
        ("Q2", "North", "Hardware", 18000.0, 85),
        ("Q2", "South", "Hardware", 12500.0, 78),
        ("Q3", "North", "Software", 16800.0, 195),
        ("Q3", "South", "Software", 13400.0, 165),
        ("Q3", "North", "Hardware", 21000.0, 92),
        ("Q3", "South", "Hardware", 14800.0, 82),
    ];

    println!("@(yellow, italic)Sales data analysis with advanced turbofish:");

    // Example 1: Multi-dimensional grouping with complex types
    let quarterly_summary: HashMap<&str, HashMap<&str, HashMap<&str, (f64, i32)>>> =
        sales_data.iter()
            .fold(
                HashMap::<&str, HashMap<&str, HashMap<&str, (f64, i32)>>>::new(),
                |mut acc, (quarter, region, product, revenue, units)| {
                    let quarter_entry = acc.entry(*quarter)
                        .or_insert_with(HashMap::<&str, HashMap<&str, (f64, i32)>>::new);
                    let region_entry = quarter_entry.entry(*region)
                        .or_insert_with(HashMap::<&str, (f64, i32)>::new);
                    let product_entry = region_entry.entry(*product).or_insert((0.0, 0));
                    product_entry.0 += *revenue;
                    product_entry.1 += *units;
                    acc
                }
            );

    println!("\n@(blue, bold)Quarterly summary (3D HashMap with turbofish):");
    for (quarter, regions) in &quarterly_summary {
        println!("@(cyan){quarter}:@()");
        for (region, products) in regions {
            println!("  @(yellow){region}:@()");
            for (product, (revenue, units)) in products {
                println!("    {product}: ${revenue:.0} revenue, {units} units");
            }
        }
    }

    // Example 2: Statistical analysis with custom structs and turbofish
    #[derive(Debug, Clone)]
    struct SalesMetrics {
        total_revenue: f64,
        total_units: i32,
        avg_price: f64,
        quarters: Vec<String>,
    }

    impl SalesMetrics {
        fn new() -> Self {
            SalesMetrics {
                total_revenue: 0.0,
                total_units: 0,
                avg_price: 0.0,
                quarters: Vec::<String>::new(),
            }
        }

        fn add_sale(&mut self, quarter: &str, revenue: f64, units: i32) {
            self.total_revenue += revenue;
            self.total_units += units;
            if !self.quarters.contains(&quarter.to_string()) {
                self.quarters.push(quarter.to_string());
            }
            self.avg_price = self.total_revenue / self.total_units as f64;
        }

        fn performance_category(&self) -> &str {
            if self.avg_price > 150.0 { "Premium" }
            else if self.avg_price > 100.0 { "Standard" }
            else { "Budget" }
        }
    }

    let product_metrics: HashMap<&str, SalesMetrics> = sales_data.iter()
        .fold(
            HashMap::<&str, SalesMetrics>::new(),
            |mut acc, (quarter, _region, product, revenue, units)| {
                let metrics = acc.entry(*product).or_insert_with(SalesMetrics::new);
                metrics.add_sale(quarter, *revenue, *units);
                acc
            }
        );

    println!("\n@(blue, bold)Product metrics analysis:");
    for (product, metrics) in &product_metrics {
        println!("@(yellow){product}@(): ${metrics.total_revenue:.0} total, {metrics.total_units} units, ${metrics.avg_price:.2} avg price, {metrics.performance_category()} category");
    }

    // Example 3: Complex functional pipeline with multiple turbofish
    let regional_performance: Vec<(String, f64, f64, &str)> = ["North", "South"].iter()
        .map(|&region| {
            let region_data: Vec<(f64, i32)> = sales_data.iter()
                .filter(|(_q, r, _p, _rev, _u)| *r == region)
                .map(|(_q, _r, _p, rev, units)| (*rev, *units))
                .collect::<Vec<(f64, i32)>>();

            let total_revenue: f64 = region_data.iter().map(|(rev, _)| *rev).sum::<f64>();
            let total_units: i32 = region_data.iter().map(|(_, units)| *units).sum::<i32>();
            let avg_price = total_revenue / total_units as f64;

            let performance = if avg_price > 130.0 { "Excellent" }
            else if avg_price > 110.0 { "Good" }
            else { "Average" };

            (region.to_string(), total_revenue, avg_price, performance)
        })
        .collect::<Vec<(String, f64, f64, &str)>>();

    println!("\n@(blue, bold)Regional performance analysis:");
    for (region, revenue, avg_price, performance) in &regional_performance {
        println!("@(cyan){region}@(): ${revenue:.0} total, ${avg_price:.2} avg price, {performance} performance");
    }

    // Example 4: Time series analysis with BTreeMap for ordered data
    let quarterly_trends: BTreeMap<&str, (f64, f64)> = sales_data.iter()
        .fold(
            BTreeMap::<&str, (f64, i32)>::new(),
            |mut acc, (quarter, _region, _product, revenue, units)| {
                let entry = acc.entry(*quarter).or_insert((0.0, 0));
                entry.0 += *revenue;
                entry.1 += *units;
                acc
            }
        )
        .into_iter()
        .map(|(quarter, (revenue, units))| (quarter, (revenue, revenue / units as f64)))
        .collect::<BTreeMap<&str, (f64, f64)>>();

    println!("\n@(blue, bold)Quarterly trends (BTreeMap for chronological order):");
    let quarters: Vec<&str> = quarterly_trends.keys().cloned().collect::<Vec<&str>>();
    for (i, (quarter, (revenue, avg_price))) in quarterly_trends.iter().enumerate() {
        let trend = if i > 0 {
            let prev_quarter = quarters[i-1];
            let prev_revenue = quarterly_trends[prev_quarter].0;
            let growth = (revenue - prev_revenue) / prev_revenue * 100.0;
            format!("({growth:+.1}%)")
        } else {
            "(baseline)".to_string()
        };
        println!("@(yellow){quarter}@(): ${revenue:.0} revenue, ${avg_price:.2} avg price {trend}");
    }

    // Example 5: Advanced filtering and aggregation with custom predicates
    let high_value_segments: HashMap<String, Vec<String>> = sales_data.iter()
        .filter(|(_q, _r, _p, revenue, _u)| *revenue > 15000.0)
        .map(|(quarter, region, product, revenue, units)| {
            let segment = format!("{}-{}", region, product);
            let description = format!("{}: ${:.0} ({} units)", quarter, revenue, units);
            (segment, description)
        })
        .fold(
            HashMap::<String, Vec<String>>::new(),
            |mut acc, (segment, description)| {
                acc.entry(segment).or_insert_with(Vec::<String>::new).push(description);
                acc
            }
        );

    println!("\n@(blue, bold)High-value segments analysis (>$15k revenue):");
    for (segment, periods) in &high_value_segments {
        let period_count = periods.len();
        let period_list = periods.join(", ");
        println!("@(cyan){segment}@(): {period_count} periods - {period_list}");
    }

    // Example 6: Statistical operations with iterator chains
    let price_distribution: Vec<(&str, f64, f64, f64)> = ["Software", "Hardware"].iter()
        .map(|&product| {
            let prices: Vec<f64> = sales_data.iter()
                .filter(|(_q, _r, p, _rev, _u)| *p == product)
                .map(|(_q, _r, _p, revenue, units)| revenue / *units as f64)
                .collect::<Vec<f64>>();

            let min_price = prices.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_price = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let avg_price = prices.iter().sum::<f64>() / prices.len() as f64;

            (product, min_price, max_price, avg_price)
        })
        .collect::<Vec<(&str, f64, f64, f64)>>();

    println!("\n@(blue, bold)Price distribution analysis:");
    for (product, min_price, max_price, avg_price) in &price_distribution {
        println!("@(yellow){product}@(): ${min_price:.2} - ${max_price:.2} range, ${avg_price:.2} average");
    }

    // Example 7: Cross-tabulation with nested HashMaps
    let cross_tab: HashMap<&str, HashMap<&str, f64>> = sales_data.iter()
        .fold(
            HashMap::<&str, HashMap<&str, f64>>::new(),
            |mut acc, (_quarter, region, product, revenue, _units)| {
                let region_entry = acc.entry(*region)
                    .or_insert_with(HashMap::<&str, f64>::new);
                *region_entry.entry(*product).or_insert(0.0) += *revenue;
                acc
            }
        );

    println!("\n@(blue, bold)Cross-tabulation: Region × Product Revenue:");
    let _products = ["Software", "Hardware"];
    println!("Region     | Software    | Hardware    | Total");
    println!("-----------|-------------|-------------|----------");

    for region in ["North", "South"] {
        let software_total = cross_tab.get(region)
            .and_then(|products| products.get("Software"))
            .unwrap_or(&0.0);
        let hardware_total = cross_tab.get(region)
            .and_then(|products| products.get("Hardware"))
            .unwrap_or(&0.0);
        let region_total = software_total + hardware_total;

        println!("{region:<10} | ${software_total:<10.0} | ${hardware_total:<10.0} | ${region_total:<9.0}");
    }

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 8.5: TURBOFISH MASTERY AND GENERIC PROGRAMMING */
    println!("@(green, bold)8.5. Turbofish mastery and generic programming:");

    // Example 1: Generic data transformation pipeline
    fn analyze_data<T, F, R>(data: &[T], transformer: F) -> Vec<R>
    where
        F: Fn(&T) -> R,
    {
        data.iter().map(transformer).collect::<Vec<R>>()
    }

    // Debug: vérifier le contenu de l'analyse générique
    let numeric_analysis = analyze_data(&sales_data, |(quarter, region, product, revenue, units)| {
        let efficiency = *revenue / *units as f64;
        format!("{} {} {}: efficiency {:.2}", quarter, region, product, efficiency)
    });

    println!("@(yellow)Generic analysis results (first 3):@()");
    let first_three: Vec<String> = numeric_analysis.iter().take(3).cloned().collect();
    for (_i, analysis_result) in first_three.iter().enumerate() {
        println!("  • {analysis_result}");
    }

    // Example 2: Complex aggregation with multiple type parameters
    fn group_and_aggregate<K, V, F, R>(
        data: &[(K, V)],
        key_fn: F
    ) -> HashMap<R, Vec<V>>
    where
        K: Clone,
        V: Clone,
        F: Fn(&K) -> R,
        R: Eq + std::hash::Hash,
    {
        data.iter()
            .map(|(k, v)| (key_fn(k), v.clone()))
            .fold(HashMap::<R, Vec<V>>::new(), |mut acc, (key, value)| {
                acc.entry(key).or_insert_with(Vec::<V>::new).push(value);
                acc
            })
    }

    let product_grouping = group_and_aggregate(
        &sales_data.iter().map(|(q, r, p, rev, u)| ((*q, *r, *p), (*rev, *u))).collect::<Vec<_>>(),
        |(quarter, _region, product)| format!("{}-{}", quarter, product)
    );

    println!("\n@(yellow)Generic grouping (first 3 groups):@()");
    for (group, values) in product_grouping.iter().take(3) {
        let total_rev: f64 = values.iter().map(|(rev, _)| *rev).sum::<f64>();
        let entry_count = values.len();
        println!("  {group}: {entry_count} entries, ${total_rev:.0} total");
    }

    // Example 3: Iterator chain with multiple collect operations and type inference
    let complex_analysis: (Vec<String>, HashMap<String, f64>, BTreeSet<i32>) = sales_data.iter()
        .fold(
            (Vec::<String>::new(), HashMap::<String, f64>::new(), BTreeSet::<i32>::new()),
            |(mut names, mut revenues, mut units), (quarter, region, product, revenue, unit)| {
                names.push(format!("{}-{}-{}", quarter, region, product));
                *revenues.entry(format!("{}-{}", region, product)).or_insert(0.0) += revenue;
                units.insert(*unit);
                (names, revenues, units)
            }
        );

    println!("\n@(yellow)Complex tuple analysis:@()");
    let combinations_count = complex_analysis.0.len();
    let revenue_streams_count = complex_analysis.1.len();
    let unique_units_count = complex_analysis.2.len();
    println!("  Product combinations: {combinations_count}");
    println!("  Revenue streams: {revenue_streams_count}");
    println!("  Unique unit values: {unique_units_count}");

    // Example 4: Custom trait implementation with turbofish
    #[allow(dead_code)]
    trait Analyzable<T> {
        fn analyze(&self) -> T;
        fn summary(&self) -> String;
    }

    impl Analyzable<(f64, f64, usize)> for Vec<(f64, i32)> {
        fn analyze(&self) -> (f64, f64, usize) {
            let total_revenue = self.iter().map(|(rev, _)| *rev).sum::<f64>();
            let avg_revenue = total_revenue / self.len() as f64;
            (total_revenue, avg_revenue, self.len())
        }

        fn summary(&self) -> String {
            let (total, avg, count) = self.analyze();
            format!("${:.0} total, ${:.2} average, {} records", total, avg, count)
        }
    }

    let software_sales: Vec<(f64, i32)> = sales_data.iter()
        .filter(|(_q, _r, p, _rev, _u)| *p == "Software")
        .map(|(_q, _r, _p, rev, units)| (*rev, *units))
        .collect::<Vec<(f64, i32)>>();

    println!("\n@(yellow)Custom trait analysis:@()");
    let software_summary = software_sales.summary();
    println!("  Software sales: {software_summary}");

    // Example 5: Advanced turbofish with closures and higher-order functions
    let create_analyzer = |threshold: f64| {
        move |data: &[(&str, &str, &str, f64, i32)]| -> HashMap<String, Vec<String>> {
            data.iter()
                .filter(|(_, _, _, rev, _)| *rev > threshold)
                .map(|(q, r, p, rev, u)| {
                    (format!("{}-{}", r, p), format!("{}: ${:.0} ({} units)", q, rev, u))
                })
                .fold(HashMap::<String, Vec<String>>::new(), |mut acc, (key, value)| {
                    acc.entry(key).or_insert_with(Vec::<String>::new).push(value);
                    acc
                })
        }
    };

    let high_value_analyzer = create_analyzer(12000.0);
    let premium_sales = high_value_analyzer(&sales_data);

    println!("\n@(yellow)Higher-order function analysis (>12k revenue):@()");
    for (segment, records) in &premium_sales {
        let quarter_count = records.len();
        println!("  {segment}: {quarter_count} high-value quarters");
    }

    // Example 6: Chained transformations with explicit type annotations
    let transformation_chain: Vec<(String, f64, String)> = sales_data.iter()
        .map(|(q, r, p, rev, _u)| (format!("{}-{}", q, r), *rev, p.to_string()))
        .collect::<Vec<(String, f64, String)>>()
        .into_iter()
        .filter(|(_period, rev, _product)| *rev > 10000.0)
        .collect::<Vec<(String, f64, String)>>()
        .into_iter()
        .map(|(period, rev, product)| {
            let category = if rev > 15000.0 { "Premium" } else { "Standard" };
            (period, rev, format!("{} ({})", product, category))
        })
        .collect::<Vec<(String, f64, String)>>();

    println!("\n@(yellow)Transformation chain results:@()");
    for (period, revenue, product_cat) in transformation_chain.iter().take(4) {
        println!("  {period}: ${revenue:.0} - {product_cat}");
    }

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 9: FORMAT SHOWCASE AND COMPARISON */
    println!("@(green, bold)9. Format showcase and comparison:");

    // Show all formats on same data
    let demo_matrix = json!([[1, 2, 3], [4, 5, 6]]);
    let demo_matrix_native = vec![vec![1, 2, 3], vec![4, 5, 6]]; // Native pour :m
    let demo_object = json!({"Name": "Alice", "Age": 25, "City": "Paris"});

    println!("@(cyan, bold)Matrix formatting comparison:");
    println!("@(yellow)Standard:@() {demo_matrix}");
    println!("@(yellow):c (compact):@() {demo_matrix:c}");
    println!("@(yellow):a (array):@() \n{demo_matrix:a}");
    println!("@(yellow):m (matrix):@() \n{demo_matrix_native:m}");
    println!("@(yellow):t (table):@() \n{demo_matrix:t}");

    println!("\n@(cyan, bold)Object formatting comparison:");
    println!("@(yellow)Standard:@() {demo_object}");
    println!("@(yellow):c (compact):@() {demo_object:c}");
    println!("@(yellow):j (pretty):@() \n{demo_object:j}");
    println!("@(yellow):t (table):@() \n{demo_object:t}");
    println!("@(yellow):t with headers:@() \n{demo_object:t(Property, Value)}");

    println!("\n@(bright_white, bold)------------------------------------------------\n");

    /* SECTION 10: BEST PRACTICES SUMMARY */
    println!("@(green, bold)10. Best practices summary:");
    println!("@(bright_blue, bold)Use JSON (json!) for:");
    println!("  • @(cyan)Static data structures and demonstrations");
    println!("  • @(cyan)Clean, readable data declarations");
    println!("  • @(cyan)Complex nested structures for display");
    println!("  • @(cyan)Table formatting examples");

    println!("\n@(bright_green, bold)Use Native Collections for:");
    println!("  • @(yellow)Data analysis and computation");
    println!("  • @(yellow)Operations like .iter(), .get(), .insert()");
    println!("  • @(yellow)Performance-critical code");
    println!("  • @(yellow)Type safety and compile-time checks");

    println!("\n@(bright_magenta, bold)Format Specifiers:");
    println!("  • @(cyan):t@() - Smart table formatting (recommended for most data)");
    println!("  • @(green):m@() - Mathematical matrix notation with Unicode brackets");
    println!("  • @(yellow):d@() - Determinant notation with vertical bars");
    println!("  • @(blue):a@() - Array format with proper indentation");
    println!("  • @(red):c@() - Compact single-line format");
    println!("  • @(magenta):j@() - JSON pretty-print format");

    println!("\n@(bright_cyan, bold, italic)=== PyRust: The Perfect Blend of JSON Convenience and Native Power ===");
}