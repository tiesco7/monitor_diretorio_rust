# Monitor de arquivos de áudio em pastas, inserção sql

## Português

### Visão Geral
Este aplicativo em Rust monitora um diretório específico para novos arquivos de áudio (`.mp3` ou `.wav`) e insere metadados extraídos de seus caminhos de arquivo em um banco de dados SQL Server. Ele utiliza o crate `notify` para monitoramento do sistema de arquivos, `odbc` para conectividade com o banco de dados e `regex` para análise dos componentes do caminho do arquivo.

### Funcionalidades
- Monitora um diretório recursivamente para eventos de criação de arquivos.
- Extrai data, número de telefone e detalhes da operação do caminho do arquivo.
- Insere os dados extraídos em uma tabela do banco de dados SQL Server (`TESTE.dbo.TABELA`).
- Registra inserções bem-sucedidas com carimbo de data/hora e contador incremental.

### Dependências
- `notify`: Para monitoramento de diretórios.
- `odbc` e `odbc_safe`: Para conectividade com SQL Server.
- `chrono`: Para análise de datas e geração de carimbos de data/hora.
- `std::sync::atomic`: Para incremento de contador seguro entre threads.
- `regex`: Para extrair nomes de operações com letras.

### Como Funciona
1. **Monitoramento de Diretório**: Usa `notify` para monitorar o diretório `\\127.0.0.1\pasta` em busca de novos arquivos.
2. **Filtragem de Arquivos**: Processa apenas arquivos `.mp3` e `.wav`.
3. **Extração de Dados**:
   - **Data**: Analisa uma data de 8 dígitos (ex.: `20230322`) do nome do arquivo para o formato `YYYY-MM-DD`.
   - **Telefone**: Extrai um número de 10 dígitos ou menor do caminho.
   - **Operação**: Extrai um segmento contendo letras do caminho.
4. **Inserção no Banco de Dados**: Conecta-se a um banco SQL Server em `127.0.0.1` e insere os dados na tabela `TESTE.dbo.TABELA`.
5. **Registro**: Exibe no console a contagem de inserções, carimbo de data/hora e caminho do arquivo.

### Estrutura do Código
- `main()`: Configura o monitoramento, conexão com o banco e loop de eventos.
- `extrair_dados()`: Extrai metadados do caminho do arquivo.
- `extrair_data()`: Analisa a data do nome do arquivo.
- `extrair_tel()`: Extrai o número de telefone.
- `extrair_operacao()`: Extrai o nome da operação usando regex.
- `inserir()`: Insere os dados no banco e registra o resultado.

### Uso
1. Certifique-se de que o SQL Server está rodando em `127.0.0.1` com o banco `TESTE` e a tabela `TABELA`.
2. Atualize a string de conexão em `main()` com suas credenciais.
3. Execute o programa: `cargo run`.
4. Adicione arquivos `.mp3` ou `.wav` ao diretório `\\127.0.0.1\pasta` para acionar o processamento.

---

# Audio File Watcher and Database Inserter

## English

### Overview
This Rust application monitors a specified directory for new audio files (`.mp3` or `.wav`) and inserts metadata extracted from their file paths into a SQL Server database. It uses the `notify` crate for file system watching, `odbc` for database connectivity, and `regex` for parsing file path components.

### Features
- Watches a directory recursively for file creation events.
- Extracts date, phone number, and operation details from the file path.
- Inserts extracted data into a SQL Server database table (`TESTE.dbo.TABELA`).
- Logs successful insertions with a timestamp and incremental counter.

### Dependencies
- `notify`: For directory watching.
- `odbc` and `odbc_safe`: For SQL Server connectivity.
- `chrono`: For date parsing and timestamp generation.
- `std::sync::atomic`: For thread-safe counter increment.
- `regex`: For extracting operation names with letters.

### How It Works
1. **Directory Monitoring**: Uses `notify` to watch the directory `\\127.0.0.1\pasta` for new files.
2. **File Filtering**: Processes only `.mp3` and `.wav` files.
3. **Data Extraction**: 
   - **Date**: Parses an 8-digit date (e.g., `20230322`) from the file name into `YYYY-MM-DD` format.
   - **Phone Number**: Extracts a 10-digit or shorter number from the path.
   - **Operation**: Extracts a segment containing letters from the path.
4. **Database Insertion**: Connects to a SQL Server database at `127.0.0.1` and inserts the data into the `TESTE.dbo.TABELA` table.
5. **Logging**: Outputs the insertion count, timestamp, and file path to the console.

### Code Structure
- `main()`: Sets up the watcher, database connection, and event loop.
- `extrair_dados()`: Extracts metadata from the file path.
- `extrair_data()`: Parses the date from the file name.
- `extrair_tel()`: Extracts the phone number.
- `extrair_operacao()`: Extracts the operation name using regex.
- `inserir()`: Inserts data into the database and logs the result.

### Usage
1. Ensure SQL Server is running at `127.0.0.1` with the database `TESTE` and table `TABELA`.
2. Update the connection string in `main()` with your credentials.
3. Run the program: `cargo run`.
4. Add `.mp3` or `.wav` files to `\\127.0.0.1\pasta` to trigger processing.

---