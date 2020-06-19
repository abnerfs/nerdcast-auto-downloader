# NerdCast Auto Downloader
Programa em Rust que acessa a API do JovemNerd, lista os últimos episódios e baixa. O programa salva a lista de episódios baixados em um arquivo JSON para não baixar de novo a próxima vez que o programa for executado.

Esse foi meu primeiro programa em Rust, era um Hello World mas resolvi fazer algo mais interessante pra ter um motivo pra aprender, com certeza o código pode ser melhorado e eu estou aberto à sugestões e Pull Requests.

# Configuração
A configuração é feita através de variáveis de ambiente, por padrão são baixados os 4 últimos episódios na pasta **./data** (Caminho relativo à onde está o executável)

# Variáveis:
 - **NERDCAST_DOWNLOAD_PATH**: Caminho onde os nerdcasts serão baixados, padrão "/data/"
 - **NERDCAST_DOWNLOAD_AMOUNT**: Quantos nerdcasts serão baixados, padrão 4
 


