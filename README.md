# Gelget

**Gelget** — это программа, предназначенная для парсинга изображений с сайта Gelbooru по заданным тегам и сохранения ссылок на них в корневом каталоге пользователя.

## Как это работает

Запустите программу с помощью команды:

```bash
$ gelget
```
После этого вам будет предложено ввести теги. Например:
```
Введите теги (например, skirt highres):
hoshino_(blue_archive) highres lying
```

Программа обработает запрос и сохранит ссылки в текстовом файле. Вы увидите сообщение о том, где были сохранены ссылки:
Ссылки записаны в: `"/home/username/file_urls_yours_tags.txt"`

Чтобы просмотреть сохраненные ссылки вы можете использовать `cat` или какой нибудь текстовый редактор:
```bash
$ cat "/home/username/file_urls_your_tags.txt"
```
или
```bash
$ nano "/home/username/file_urls_your_tags.txt"
```
или
```bash
$ vim "/home/username/file_urls_your_tags.txt"
```
или 
что-то своё

Вывод будет содержать ссылки на изображения, например:
```
https://img3.gelbooru.com/images/49/67/4967fa4777eb407a23cf99cb3a2a799c.jpg
https://img3.gelbooru.com/images/6e/11/6e11c424826e17e4dbd726f7958c05b2.png
https://img3.gelbooru.com/images/c7/37/c73704d5c68f5abba4ad4510d9d6062d.jpg
https://img3.gelbooru.com/images/77/0b/770b5183506a3bbfaaf88670007f2089.jpg
https://img3.gelbooru.com/images/a5/bb/a5bb1083f59b878a97f69f16a1b88248.png
```
## Вопросы по работе
проверялся только на Arch linux на других дитриьутивах работу я гарантировать не могу
а тем более на Windows))

# Гайд по сборке Gelget
требования:
- rust
- доступ в интернет
- cargo

сборка:
```bash
$ git clone https://github.com/Snappus/gelget.git
$ cd gelget
$ cargo built --release
$ ./target/release/gelget
```

# Будущее для Gelget

В планах на будущее:

- Поддержка нескольких языков для удобства пользователей.
- Разработка более удобного интерфейса для улучшения взаимодействия с программой.

## P.S
для жителей россии придется использовать vpn)
