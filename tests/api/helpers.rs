use regex::Regex;

pub async fn spawn_app() {
    let server = actix_rest_vue_setup::run().expect("Failed to bind address.");
    let _ = tokio::spawn(server);
}

pub fn get_index_matching_re() -> Regex {
    Regex::new(
        r#"<!DOCTYPE html>
<html lang="en">
 {2}<head>
 {4}<meta charset="UTF-8">
 {4}<link rel="icon" href="/favicon\.ico">
 {4}<meta name="viewport" content="width=device-width, initial-scale=1\.0">
 {4}<title>Vite App</title>
 {4}<script type="module" crossorigin src="/assets/index-[0-9a-f]+\.js"></script>
 {4}<link rel="stylesheet" href="/assets/index-[0-9a-f]+\.css">
 {2}</head>
 {2}<body>
 {4}<div id="app"></div>
 {4}
 {2}</body>
</html>
"#,
    ).unwrap()
}
