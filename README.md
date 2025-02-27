## cliser A client/server application to learn Rust
<pre>
┌───────────────────┐              ┌───────────────┐
│   tcp_reader      │              |  Backdoor     │
│                   │              │               │
│                   │              │               │
│                   │              └───────────────┘
│                   │                 |     |
│                   │                 |     |
│                   │                 |     |
│                   │                 |     |
│                   │              ┌───────────────┐
│                   │─────────────>│    Manager    │
│                   │<─────────────│               │
└───────────────────┘              │               │
                                   │               │
┌───────────────────┐              │               │
│   tcp_writer      │─────────────>│               │
│                   │<─────────────│               │
│                   │              │               │
│                   │              │               │
│                   │              └───────────────┘
│                   │
│                   │
│                   │
│                   │
└───────────────────┘
</pre>

Thread Manager gerência as conexões.

A função main põe o Servidor no ar e fica ouvindo a porta configurada para o sistema.

Quando algum cliente se conecta ao servidor a seguinte sequência acontece:

1 - A thread conection_manager:

1.1 - Aceita a conexão:

1.1.0 - Cria os mailbox para conversar com as threads handle_backdoor,tcp writer e reader

1.1.0.1 - Cria sender,receiver para falar com a tcp_writer

1.1.0.2 - Cria sender,receiver para falar com a tcp_reader

1.1.1 - Cria a thread tcp_writer, passando o stream do tcp e a msgqueue tcp_writer.receiver

1.1.2 - Cria a thread tcp_reader, passando o stream do tcp e a msgqueue tcp.reader.sender

1.1.3 - Cria a thread auth_manager

1.1.3.1 - Envia para a  msgqueue tcp_writer.receiver uma mensagem,110, para pedir a senha para o cliente.

1.1.3.2 - Aguarda a resposta, a senha, na msgqueue tcp_reader.receiver a senha. Valida a senha e   se for inválida desconecta o cliente e volta a ouvir o stream tcp/ip.

1.1.3.3 - Enviao uma mensagem,120, para pedir o Id do cliente, para a thread_writer via msgqueue.

1.1.3.4 - Aguarda a resposta,o ID  do cliente, na msgqueue tcp_reader.receiver.
1.1.3.4.1 - Verifica se o cliente já exites no sistema.
            se o cliente existir envia mensagem de desconecção e termina a thread.

1.1.3.5 - Extrai da stream conexão tcp/ip o IP e Porta do cliente

1.1.3.6 - Cadastra o cliente. na estrutura

pub struct ClientData {
    pub id: u16,
    pub ip: String,
    pub status: String, // "active" or "inactive"
    pub port: String, // Porta do cliente
    pub cid: String, // ID do cliente
    pub sender_tcp_writer: mpsc::Sender<String>, // Sender para enviar mensagens ao cliente
}

1.1.3.7 - Envia para a thread_writer via msgqueue uma mensagem,140, de cliente conectado.

1.2 - Entra no loop principal, e fica tratando as mensagems que entram e que saem.
      * Envia um keep_alive a cada 10 segundos.

Formação dos comandos: código: descrição

Os comandos são sempre enviados pelo servidor o cliente somente responde.
<pre>
100: keep alive
110: qual a senha
120: qual o ID
130: shutdown sys
140: Connected
999: shutdown
</pre>

<pre>
Comandos recebidos pela backdoor
200: Envia keep alive
201: Comando para backdoor enviar comando de
     desconexao para todos os clientes
202:1000 Comando para backdoor enviaw um 
         comando de desconexão para o cliente.
         Formato cmd:client id
</pre>














#raspberry#ds1820#rust#server#client#tcp#tcpip#tcp/ip#connection#raspberrypi #python #raspberrypi5
#python #vscode #raspberrypi5 #linuxubuntu #ubuntu #raspberrypi5 #raspberrypi #machinelearning #robotics #iot #edgecomputing #edge #aleksandarhaber #arduino #arduinoIDE #iot