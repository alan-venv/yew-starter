# Single Page Application (SPA)

## O que é

Uma SPA é uma aplicação web que carrega uma única página HTML e atualiza o conteúdo dinamicamente via JavaScript (ou WebAssembly), sem recarregar a página inteira a cada navegação. O browser assume o papel de runtime da aplicação enquanto o servidor passa a ser apenas uma API de dados.

No modelo tradicional (Multi Page Application), cada interação do usuário dispara um ciclo HTTP completo: request ao servidor, renderização do HTML no backend, e resposta com uma página inteira nova. Na SPA, o servidor entrega a aplicação uma vez e toda navegação subsequente acontece no client.

## Possibilidades que o modelo SPA abre

- **Estado persistente entre views** -- um player de música que continua tocando enquanto o usuário navega (Spotify Web), ou um chat aberto em qualquer página (Intercom).
- **Atualizações em tempo real** -- dashboards com WebSocket que atualizam dados sem refresh (Grafana, Figma colaborativo).
- **Interações offline** -- com Service Workers, a aplicação pode funcionar sem conexão e sincronizar depois (Google Docs, Notion).
- **Transições e animações contínuas** -- arrastar itens entre listas (Trello), reordenar elementos com drag-and-drop, canvas interativos.
- **Lazy loading granular** -- carregar módulos sob demanda conforme o usuário navega, ao invés de tudo de uma vez.

## SPA vs MPA

| Aspecto | SPA | MPA (tradicional) |
|---|---|---|
| Navegação | Client-side (History API) | Server-side (reload completo) |
| Estado | Persiste entre rotas | Perdido a cada request |
| Servidor | API de dados | Renderiza HTML |
| Primeira carga | Mais pesada | Mais leve |
| Navegações seguintes | Instantâneas | Novo ciclo HTTP |
| SEO | Requer SSR/prerender | Nativo |
| Offline | Possível (Service Workers) | Não viável |

## WASM como diferencial

WebAssembly adiciona uma camada extra à SPA: performance próxima de nativo no browser. Aplicações que antes exigiam desktop agora rodam no browser com performance viável -- editores de imagem, ferramentas CAD, processamento pesado de dados. Frameworks como Yew permitem construir SPAs em Rust compilando para WASM, combinando segurança de memória com performance.
