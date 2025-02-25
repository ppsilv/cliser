## cliser A client/server application to learn Rust
<pre>
┌───────────────────┐
│   tcp_reader      │
│                   │
│                   │
│                   │
│                   │
│                   │
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

1.1.0 - Cria os mailbox para conversar com as threads tcp writer e reader

1.1.0.1 - Cria sender,receiver para falar com a tcp_writer

1.1.0.2 - Cria sender,receiver para falar com a tcp_reader

1.1.1 - Cria a thread tcp_writer, passando o stream do tcp e a msgqueue tcp_writer.receiver

1.1.2 - Cria a thread tcp_reader, passando o stream do tcp e a msgqueue tcp.reader.sender

1.1.3 - Cria a thread ger_msg

1.1.3.1 - Envia para a  msgqueue tcp_writer.receiver um comando para pedir a senha para o cliente.

1.1.3.2 - Aguarda na msgqueue tcp_reader.receiver a senha. Valida a senha e se for inválida desconecta 
          o cliente e volta a ouvir o stream tcp/ip.

1.1.3.3 - Envia para a thread_writer via msgqueue um comando para pedir o Id do cliente.

1.1.3.4 - Aguarda na msgqueue tcp_reader.receiver o ID do cliente

1.1.3.5 - Extrai da conexão tcp/ip o IP e Porta do cliente

1.1.3.6 - Cadastra o cliente. na estrutura

      clientedata{
        sid: u16,
        cid: String,
        cip: String,
        cport: String,
        cstatus: bool, //true = conectado false = desconectado
      }

1.1.3.7 - Envia para a thread_writer via msgqueue um aviso de cliente conectado.

Formação dos comandos: código: descrição

Os comandos são sempre enviados pelo servidor o cliente somente responde.
<pre>
100: keep alive
110: qual a senha
120: qual o ID
130: shutdown sys
140: Connected
</pre>
