# hitoku
`hitoku` は簡易的なjinja(minijinja)のラッパーツールです。  
環境変数から値を読み取ってテンプレートの値を埋め、ファイルを生成します。  

## 使い方
### テンプレートを用意する
環境変数から埋めたい箇所を `{{ env.ENV_VAR_NAME }}` 形式で書きます。  
環境変数名の前に、必ず `env.` を付ける必要があります。  

```example.toml.tmpl
name = "example-file"

[config]
lang = ja

[secrets]
database_endpoint = {{ env.DB_ENDPOINT }}
database_user = {{ env.DB_USER }}
database_password = {{ env.DB_PASSWORD }}
database_name = {{ env.DB_NAME }}

```

### コマンドを実行する
```zsh
# hitoku generate [tempalte] [output]
hitoku generate ./example.toml.tmpl example.toml
```

他の使い方は `--help` を参照してください。  

---

**Note: This document was translated by a generative AI.**  

# hitoku
`hitoku` is a simple wrapper tool for jinja (minijinja).  
It reads values from environment variables to populate template values and generate files.  

## Usage
### Prepare the template
Write the parts to be filled from environment variables in the format `{{ env.ENV_VAR_NAME }}`.  
You must prefix the environment variable name with `env.`.  

```example.toml.tmpl
name = "example-file"

[config]
lang = ja

[secrets]
database_endpoint = {{ env.DB_ENDPOINT }}
database_user = {{ env.DB_USER }}
database_password = {{ env.DB_PASSWORD }}
database_name = {{ env.DB_NAME }}
```

### Execute the command
```zsh
# hitoku generate [template] [output]
hitoku generate ./example.toml.tmpl example.toml
```

For other usage, refer to `--help`.  
