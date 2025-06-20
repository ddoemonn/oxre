use clap::{Arg, Command};
use colored::*;
use anyhow::{Result, Context};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};


mod templates;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("oxre")
        .about("⚡ Blazingly fast React project scaffolder")
        .version("0.1.0")
        .arg(
            Arg::new("name")
                .help("Name of the React project")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("template")
                .long("template")
                .short('t')
                .help("Template to use")
                .value_name("TEMPLATE")
                .default_value("default")
        )
        .arg(
            Arg::new("typescript")
                .long("typescript")
                .short('T')
                .help("Use TypeScript template")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .help("Target directory")
                .value_name("DIR")
        )
        .get_matches();

    let project_name = matches.get_one::<String>("name").unwrap();
    let template = if matches.get_flag("typescript") {
        "typescript"
    } else {
        matches.get_one::<String>("template").unwrap()
    };
    
    let target_dir = if let Some(dir) = matches.get_one::<String>("dir") {
        PathBuf::from(dir).join(project_name)
    } else {
        PathBuf::from(project_name)
    };

    println!("🚀 {}", "Creating React project with oxre...".bright_cyan().bold());
    println!("📦 Project: {}", project_name.bright_green());
    println!("📋 Template: {}", template.bright_yellow());
    println!("📁 Location: {}", target_dir.display().to_string().bright_blue());
    println!();

    // Check if directory exists
    if target_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory '{}' already exists!", 
            target_dir.display()
        ));
    }

    // Create project
    create_project(&target_dir, project_name, template).await?;
    
    println!("✅ {}", "Project created successfully!".bright_green().bold());
    println!();
    println!("🏃 Next steps:");
    println!("  cd {}", project_name.bright_cyan());
    println!("  npm install");
    println!("  npm run dev");
    println!();
    println!("🎉 Happy coding!");

    Ok(())
}

async fn create_project(target_dir: &Path, project_name: &str, template: &str) -> Result<()> {
    // Create project directory
    fs::create_dir_all(target_dir)
        .context("Failed to create project directory")?;

    // Generate files based on template
    match template {
        "typescript" => create_typescript_project(target_dir, project_name).await?,
        "default" => create_default_project(target_dir, project_name).await?,
        _ => return Err(anyhow::anyhow!("Unknown template: {}", template)),
    }

    Ok(())
}

async fn create_default_project(target_dir: &Path, project_name: &str) -> Result<()> {
    println!("📝 Generating files...");
    
    // Create package.json
    let package_json = json!({
        "name": project_name,
        "private": true,
        "version": "0.0.0",
        "type": "module",
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "lint": "eslint . --ext js,jsx --report-unused-disable-directives --max-warnings 0",
            "preview": "vite preview"
        },
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "devDependencies": {
            "@types/react": "^18.2.66",
            "@types/react-dom": "^18.2.22",
            "@vitejs/plugin-react": "^4.2.1",
            "eslint": "^8.57.0",
            "eslint-plugin-react": "^7.34.1",
            "eslint-plugin-react-hooks": "^4.6.0",
            "eslint-plugin-react-refresh": "^0.4.6",
            "vite": "^5.2.0"
        }
    });

    fs::write(
        target_dir.join("package.json"),
        serde_json::to_string_pretty(&package_json)?
    )?;

    // Create vite.config.js
    fs::write(
        target_dir.join("vite.config.js"),
        templates::VITE_CONFIG
    )?;

    // Create index.html
    fs::write(
        target_dir.join("index.html"),
        templates::INDEX_HTML.replace("{{PROJECT_NAME}}", project_name)
    )?;

    // Create src directory
    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    // Create main.jsx
    fs::write(src_dir.join("main.jsx"), templates::MAIN_JSX)?;

    // Create App.jsx
    fs::write(src_dir.join("App.jsx"), templates::APP_JSX)?;

    // Create App.css
    fs::write(src_dir.join("App.css"), templates::APP_CSS)?;

    // Create index.css
    fs::write(src_dir.join("index.css"), templates::INDEX_CSS)?;

    // Create public directory
    let public_dir = target_dir.join("public");
    fs::create_dir_all(&public_dir)?;

    // Create .gitignore
    fs::write(target_dir.join(".gitignore"), templates::GITIGNORE)?;

    // Create README.md
    fs::write(
        target_dir.join("README.md"),
        templates::README_MD.replace("{{PROJECT_NAME}}", project_name)
    )?;

    Ok(())
}

async fn create_typescript_project(target_dir: &Path, project_name: &str) -> Result<()> {
    println!("📝 Generating TypeScript files...");
    
    // Create package.json with TypeScript dependencies
    let package_json = json!({
        "name": project_name,
        "private": true,
        "version": "0.0.0",
        "type": "module",
        "scripts": {
            "dev": "vite",
            "build": "tsc && vite build",
            "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
            "preview": "vite preview"
        },
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "devDependencies": {
            "@types/react": "^18.2.66",
            "@types/react-dom": "^18.2.22",
            "@typescript-eslint/eslint-plugin": "^7.2.0",
            "@typescript-eslint/parser": "^7.2.0",
            "@vitejs/plugin-react": "^4.2.1",
            "eslint": "^8.57.0",
            "eslint-plugin-react-hooks": "^4.6.0",
            "eslint-plugin-react-refresh": "^0.4.6",
            "typescript": "^5.2.2",
            "vite": "^5.2.0"
        }
    });

    fs::write(
        target_dir.join("package.json"),
        serde_json::to_string_pretty(&package_json)?
    )?;

    // Create other files...
    fs::write(target_dir.join("vite.config.ts"), templates::VITE_CONFIG_TS)?;
    fs::write(target_dir.join("tsconfig.json"), templates::TSCONFIG_JSON)?;
    fs::write(target_dir.join("tsconfig.node.json"), templates::TSCONFIG_NODE_JSON)?;
    
    fs::write(
        target_dir.join("index.html"),
        templates::INDEX_HTML.replace("{{PROJECT_NAME}}", project_name)
    )?;

    let src_dir = target_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    fs::write(src_dir.join("main.tsx"), templates::MAIN_TSX)?;
    fs::write(src_dir.join("App.tsx"), templates::APP_TSX)?;
    fs::write(src_dir.join("App.css"), templates::APP_CSS)?;
    fs::write(src_dir.join("index.css"), templates::INDEX_CSS)?;
    fs::write(src_dir.join("vite-env.d.ts"), templates::VITE_ENV_DTS)?;

    let public_dir = target_dir.join("public");
    fs::create_dir_all(&public_dir)?;

    fs::write(target_dir.join(".gitignore"), templates::GITIGNORE)?;
    fs::write(
        target_dir.join("README.md"),
        templates::README_MD.replace("{{PROJECT_NAME}}", project_name)
    )?;

    Ok(())
}
