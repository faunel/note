 
# Проста консольна программа для ведення коротких нотаток:
Ця програма дозволяє робити такі речі
1. Створювати нові нотатки
2. Переглядати весь список нотаток
3. Відмічати потрібну нотатку по її номеру
4. Видаляти помічені нотатки
5. Видаляти всі нотатки


# Зміст  
1. [Компіляція](#Компіляція)  
2. [Використання](#Використання)  
3. [Приклади](#Приклади)  

# Компіляція
Виконайте наступні команди для компіляції

~~~bash  
  git clone https://link-to-project
  cd note
  cargo build --release
~~~

Потім перейдіть в каталог програми
~~~bash 
  cd target/release
~~~

# Використання

~~~bash  
Використання:
  note [SUBCOMMAND]

OPTIONS:
  -h, --help       Печатає довідку з інформацією
  -V, --version    Печатає версію програми

SUBCOMMANDS:
  add       Додає нову нотатку
  clear     Очищає помічені нотатки. Видаляє всі якщо є прапорець `-a`
  done      Помічає нотатку
  help      Печатає довідку з інформацією
  remove    Видаляє нотатку по номеру
~~~

Якщо SUBCOMMAND не передана виводить список всіх нотаток

# Приклади

Додати нові нотатки

~~~bash  
  ./note add "Hello World"
  ./note add "Прочитати Rustbook"
  ./note add "Навчитись писати на Rust"
~~~

Переглянути список нотаток

~~~bash  
  ./note
~~~

Помітити нотатку з номером 4

~~~bash  
  ./note done 4
~~~

Видалити нотатку з номером 2

~~~bash  
  ./note remove 2
~~~

Видалити помічені нотатки

~~~bash  
  ./note clear
~~~

Видалити всі нотатки

~~~bash  
  ./note clear -a
~~~