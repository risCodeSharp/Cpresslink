pub struct EmailMessage {
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}
const WEBSITE_LINK: &str = "https://riscodesharp.github.io/Cpresslink/";
pub fn welcome_email(user_name: &str) -> EmailMessage {
    let subject = format!("Welcome to C.Presslink, {}!", user_name);

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>

<body style="margin:0; padding:0; background:#F4F4F5; font-family:Arial,Helvetica,sans-serif;">
<h1>Welcome aboard, {user_name}!</h1>
<p>Your account is ready to go.</p>

<p>
You're all set! Your <strong style="color:#0F6E56;">C.Presslink</strong> account has been created.
</p>

<a href="{WEBSITE_LINK}"
   style="display:inline-block; background:#0F6E56; color:white; padding:12px 24px; text-decoration:none; border-radius:6px;">
   Create your first link
</a>

<p style="margin-top:20px; font-size:12px; color:#999;">
Didn't create this account? Ignore this email.
</p>

</body>
</html>"#,
        user_name = user_name
    );

    let text = format!(
        "Welcome to C.Presslink, {user_name}!

Your account has been created successfully.

Start here: {WEBSITE_LINK}

— The C.Presslink Team",
        user_name = user_name
    );

    EmailMessage {
        subject,
        html_body: html,
        text_body: text,
    }
}

pub fn milestone_email(user_name: &str, count: i64) -> EmailMessage {
    let subject = format!("You've created {} links on C.Presslink!", count);

    let (next_milestone, progress_label) = match count {
        100 => ("1,000", "You're just getting started — 1,000 is next!"),
        1_000 => ("10,000", "Four digits in! Keep pushing toward 10,000."),
        10_000 => ("—", "You've hit the top milestone. Absolutely legendary."),
        _ => ("100", "Keep going — your first milestone is just ahead!"),
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>

<body style="margin:0; padding:0; background:#F4F4F5; font-family:Arial,Helvetica,sans-serif;">

<h1>Amazing work, {user_name}!</h1>

<p>
You've created <strong style="color:#0F6E56;">{count} links</strong> on C.Presslink.
</p>

<p>{progress_label}</p>

<p><strong>Next goal:</strong> {next_milestone}</p>

<a href="{WEBSITE_LINK}"
   style="display:inline-block; background:#0F6E56; color:white; padding:12px 24px; text-decoration:none; border-radius:6px;">
   Create more links
</a>

</body>
</html>"#,
        user_name = user_name,
        count = count,
        progress_label = progress_label,
        next_milestone = next_milestone
    );

    let text = format!(
        "Amazing work, {user_name}!

You've created {count} links.

{progress_label}

Next goal: {next_milestone}

{WEBSITE_LINK}

— The C.Presslink Team",
        user_name = user_name,
        count = count,
        progress_label = progress_label,
        next_milestone = next_milestone
    );

    EmailMessage {
        subject,
        html_body: html,
        text_body: text,
    }
}