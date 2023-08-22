use colored::*;
use std::io;
use std::collections::HashMap;// to add my appointment


struct Horse {
    name: String,
    age: u32,
    breed: String,
    weight: f64,
    temperature: f64,
    height: f64,
    insured: bool,
    gender: char,
    appointments: HashMap<String, String>,
}

fn main() {
    let mut horses: Vec<Horse> = Vec::new();

    println!(
        "{}",
        "\n\n\t\t**** Welcome Egan's Shiatsu Equine Massage Therapy ****"
            .red()
    );

    println!(
        "{}",
        "\n\n\t\t   Welcome to Egan's Shiatsu Equine Massage Therapy,
         where your horse's well-being is our top priority. 
         I have 15 years of experiance specializing in massage
        treatments that promote healing, alleviate tension,
        and enhance your horse's overall comfort and performance. 
        Whether your horse is an athlete, a companion, 
        or a beloved friend, my tailored approach ensures
        that each session is designed to meet their unique needs.
        Trust in my passion for equine care and let us support
        your horse's journey towards optimal health and performance."
            .red()
            .bold()
    );
// put in a loop here where i'm getting all my horeses details so the user can add
// more then one horse
    loop {
        let mut first_name = String::new();

        println!(
            "{}",
            "\n\n\t\t  Enter Your Horse's Name\n\n"
                .bright_green()
        );

        io::stdin()
            .read_line(&mut first_name)
            .expect("Failed to read first name");
        let first_name = first_name.trim().to_string();

        println!(
            "{}",
            format!(
                "\t\t I Love the name, {}",
                first_name
            )
            .yellow()
            .bold()
            .italic()
        );

        let mut gender = String::new();

        println!(
            "{}",
            "\n\n \t\t  Mare, Gelding or Stallion? (M/G/S)\n\n"
                .bright_green()
        );

        io::stdin()
            .read_line(&mut gender)
            .expect("Failed to get horse's gender");

        let c_gender = gender.trim().chars().next().unwrap();

        println!(
            "{}",
            format!(
                "\t\t Knowing, {} gender help insure safety for all!",
                first_name
            )
            .yellow()
            .bold()
            .italic()
        );

        let mut age = String::new();

        println!(
            "{}",
            "\n\n\t\t   Age?\n\n"
                .bright_green()
        );

        io::stdin()
            .read_line(&mut age)
            .expect("Failed to read horse's age");

        let age: u32 = age.trim().parse().expect("Please type in a number!");

        println!(
            "{}",
            format!(
                "\t\t  {} is bueatiful regaudless of age!",
                first_name
            )
            .yellow()
            .bold()
            .italic()
        );
// is requiring a number to be input
        let mut temperature = String::new();

        println!(
            "{}",
            "\n\n\t\t How many sessions do you want each week? \n\n"
                .bright_green()
        );

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read horse's temperature");
        let temperature: f64 = temperature.trim().parse().expect("Please type a number!");

        println!(
            "{}",
            format!(
                "\t\t  {}, will be back to normal in no time.",
                first_name
            )
            .yellow()
            .bold()
            .italic()
        );

        let mut height = String::new();

        println!(
            "{}",
            "\n\n\t\t   Height?\n\n"
                .bright_green()
        );

        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read horse's height");
        let height: f64 = height.trim().parse().expect("Please type a number!");

        println!(
            "{}",
            format!(
                "\t\t  {}, is has long O' legs!",
                first_name
            )
            .yellow()
            .bold()
            .italic()
        );

        let mut horse = Horse {
            name: first_name.clone(),
            age: age,
            breed: String::new(),
            weight: 0.0,
            temperature: temperature,
            height: height,
            insured: false,
            gender: c_gender,
            appointments: HashMap::new(),
        };

        println!(
            "\n\n\t\t   Would you like to schedule an appointment for {}? (yes/no)\n\n",
            first_name
        );
// I wanted to allow the user to add an appointment for each horse added
//Y allows them to add  and N asks if they want to add another horse
        let mut schedule_appointment = String::new();
        io::stdin()
            .read_line(&mut schedule_appointment)
            .expect("Failed to read response");

        let schedule_appointment = schedule_appointment.trim().to_lowercase();
        if schedule_appointment == "yes" || schedule_appointment == "y" {
            println!(
                "\n\n\t\t   Please enter the appointment date and time for {} (e.g., '2023-08-31 14:00'):\n\n",
                first_name
            );

            let mut appointment_date_time = String::new();
            io::stdin()
                .read_line(&mut appointment_date_time)
                .expect("Failed to read appointment date and time");

            horse.appointments.insert(first_name.clone(), appointment_date_time.trim().to_string());
        }

        horses.push(horse);

        let mut another_horse = String::new();
// when they select no to adding another horse the program will then 
//print for records for each horse added, no limits
//and I have add confetti at the end 
        println!(
            "\n\n\t\t   Do you want to add another horse? (yes/no)\n\n"
        );

        io::stdin()
            .read_line(&mut another_horse)
            .expect("Failed to read response");

        let another_horse = another_horse.trim().to_lowercase();
        if another_horse != "yes" && another_horse != "y" {
            break;
        }
    }

    for horse in &horses {
        println!("{}", "\n\n<<< Horse Information >>>".yellow());
        println!("{}", "--------------------".yellow());
        println!("Horse's name: {}", horse.name);
        println!("Horse's gender: {}", horse.gender);
        println!("Horse's age: {}", horse.age);
        println!("Horse's temperature: {}", horse.temperature);
        println!("Horse's height: {}", horse.height);
       

        // Printing appointments
        println!("Appointments:");
        for (date, time) in &horse.appointments {
            println!("Date: {}, Time: {}", date, time);
        }
        //I have add confetti 
        let confetti = "
        .   *          .
  .     *    .   *     *   .
    *    .    *    .    *     .
    *  .    *  .   *   *   .
    *   *    .    *    *  .
       *  .  *   *    .     *   .
  .     *    *       .   *    .
         .   *  .   *     *   .
   *    .    *    .    *   .
       *  .  *   *    .  *    .
      *    *        .  *     .
 ";
 
     println!("{}", confetti.bright_yellow());
    }
    let msg: ColoredString = format!(
        "\n\n\t\t\t\t****CONGRATULATIONS YOU ARE SCHEDULED!****\n\n"
    )
    .red();
    println!("{}", msg);
    let msg: ColoredString = format!(
        "\n\n\t\t\tThank you for using Egan's Equine Massage Therapy!\n\n"
    )
    .red();
    println!("{}", msg);

    
}










