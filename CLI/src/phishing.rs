use qrcode::QrCode;
use qrcode::types::Color;
use image::{ImageBuffer, Luma};
use std::fs::File;
use std::io::{self, Write};

pub fn generate_email() {
    // Get user inputs for dynamic variables
    let subject = "Your Devices Have Been Compromised - Immediate Action Required";
    let body_template = "Dear User,

We have detected unusual activity on your account, and immediate verification is required to avoid suspension. Failure to act within 24 hours will result in restricted access to your account and possible data loss.

To resolve this issue, please scan the QR code below and follow the instructions on the secure webpage:

![QR Code Placeholder]
(This QR code redirects you to the verification portal.)

Important:

Open your phone's camera or QR code scanner app.
Scan the QR code to access the secure verification page.
Follow the instructions to verify your account activity.

This is an automated security measure to protect your data and account integrity. If you do not complete this verification process, we will assume the activity is unauthorized and take necessary action.

Thank you for your immediate attention to this matter.

Sincerely,
[Your Support Team]
[Support Email Address]";

    let qr_code_url = get_input("Enter the website URL for the QR code:");
    let support_team = get_input("Enter the name of your support team:");
    let support_email = get_input("Enter the support email address:");

    // Generate the email template
    let email_template = generate_email_template(
        subject,
        body_template,
        true,
        Some(&qr_code_url),
        &support_team,
        &support_email,
    );

    // Save the email template to a file
    let file_name = "generated_email.txt"; // You can change the file name as needed
    if let Err(e) = save_to_file(file_name, &email_template) {
        eprintln!("Failed to write to file: {}", e);
    } else {
        println!("\nGenerated email template saved to '{}'.", file_name);
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn generate_email_template(
    subject: &str,
    body_template: &str,
    include_qr: bool,
    qr_code_url: Option<&str>,
    support_team: &str,
    support_email: &str,
) -> String {
    // Start constructing the email template
    let mut email_template = format!("Subject: {}\n\n", subject);

    // Insert the body content
    let body = body_template
        .replace("[Your Support Team]", support_team)
        .replace("[Support Email Address]", support_email);

    let qr_code_placeholder = if include_qr && qr_code_url.is_some() {
        let qr_url = qr_code_url.unwrap();
        let qr_code_image_path = save_qr_code_as_image(qr_url).unwrap_or_else(|e| {
            eprintln!("Failed to save QR code as image: {}", e);
            "(QR code could not be generated)".to_string()
        });
        format!("\n![QR Code]({})\n", qr_code_image_path)
    } else {
        "\n(Scan the QR code provided above.)\n".to_string()
    };

    // Replace placeholder in the body
    let body_with_qr = body.replace("![QR Code Placeholder]", &qr_code_placeholder);

    email_template.push_str(&body_with_qr);

    email_template
}

fn save_qr_code_as_image(url: &str) -> io::Result<String> {
    let qr = QrCode::new(url).expect("Failed to generate QR code");
    let width = qr.width();
    let scale = 10; // Scale factor to increase resolution
    let scaled_width = width * scale;

    let mut image_buffer = ImageBuffer::new(scaled_width as u32, scaled_width as u32);

    for y in 0..width {
        for x in 0..width {
            let pixel = match qr[(x, y)] {
                Color::Dark => Luma([0u8]), // Black pixel
                Color::Light => Luma([255u8]), // White pixel
            };

            // Write the scaled pixel to the output image
            for dy in 0..scale {
                for dx in 0..scale {
                    image_buffer.put_pixel(
                        (x * scale + dx) as u32,
                        (y * scale + dy) as u32,
                        pixel,
                    );
                }
            }
        }
    }

    let image_path = "qr_code.png";
    image_buffer.save(image_path).expect("Failed to save QR code image");

    Ok(image_path.to_string())
}

fn save_to_file(file_name: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
