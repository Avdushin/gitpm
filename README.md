# Git Profile Manager

**Git Profile Manager** — это CLI-инструмент для удобного управления несколькими профилями Git. Если вы работаете с разными учетными записями на GitHub, GitLab или других платформах, данное приложение позволяет легко переключаться между профилями, не вводя каждый раз имя пользователя и адрес электронной почты вручную. С помощью этого инструмента вы можете добавлять, удалять и переключаться между профилями Git, что значительно упрощает работу в командах и проектах с разными учетными записями.

## Как работает

Git Profile Manager предоставляет набор команд для создания и управления профилями Git:
```
A CLI tool for managing multiple git profiles

Usage: gitpm.exe <COMMAND>

Commands:
  add         Add a new Git profile
  switch      Switch to an existing Git profile
  remove      Remove a specific Git profile (aliases: rm, -rm)
  remove-all  Remove all Git profiles
  list        List all Git profiles
  doctor      Check the configuration directory and settings (aliases: doc, -doc)
  current     Show the current Git profile
  version     Get current CLI version (aliases: -v, --version)
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Давайте исправим вложенности и форматирование вашего документа, чтобы он стал более структурированным и читабельным. Вот исправленный вариант:

## Использование

После установки вы можете использовать команду `gitpm` в терминале для запуска программы и выполнения команд. Примеры использования:

### Пример использования
```bash
# Add profile
$ gitpm add
Enter profile name or select a profile service (1 - GitHub, 2 - GitLab, 3 - Other):
Example_profile
Enter user name: example
Enter email: example@gmail.com
Enter remote URL (optional):
Profile added successfully!

# Switch profile
$ gitpm switch
Select a profile to switch to:
1. GitFlick
2. GitHub_Avdushin
3. Example_profile
4. GitLab_Avdushin
Enter the number of the profile: 3
Switched to profile 'Example_profile'

# Current profile
$ gitpm current
Current Git Profile:
---------------------
User Name: example
Email: example@gmail.com

#--- git config
$ git config user.name
example
$ git config user.email
example@gmail.com

# Profiles list
$ gitpm list

Your Profiles:
--------------
Profile Name: GitHub_Avdushin
Service_User: GitHub_Avdushin
User Name: Avdushin
Email: avdushinbeaver1@gmail.com
Remote:

Profile Name: Example_profile
Service_User: Example_profile
User Name: example
Email: example@gmail.com
Remote:

Profile Name: GitLab_Avdushin
Service_User: GitLab_Avdushin
User Name: Avdushin
Email: avdushinbeaver1@gmail.com
Remote:
```

<details>
<summary>Добавление нового профиля (gitpm add)</summary>

```bash
# Пример:
$ gitpm add
Select a profile service (1 - GitHub, 2 - GitLab, 3 - Other):
1
Enter user name: Avdushin
Enter email: avdushinbeaver1@gmail.com
Enter remote URL (optional):
Profile added successfully!
```

</details>

<details>
<summary>Получение текущего профиля (gitpm current)</summary>

```bash
# Пример:
$ gitpm current
Current Git Profile:
---------------------
User Name: Avdushin
Email: avdushinbeaver1@gmail.com
```

</details>

<details>
<summary>Переключение на другой профиль (gitpm switch)</summary>

```bash
$ gitpm switch
Select a profile to switch to:
1. GitFlick
2. GitHub_Avdushin
3. Example_profile
4. GitLab_Avdushin
Enter the number of the profile: 3
Switched to profile 'Example_profile'
#--- git config
$ git config user.name
example
$ git config user.email
example@gmail.com
```

</details>

<details>
<summary>Отображение списка всех профилей (gitpm list)</summary>

```bash
$ gitpm list

Your Profiles:
--------------
Profile Name: GitLab_Avdushin
Service_User: GitLab_Avdushin
User Name: Avdushin
Email: avdushinbeaver1@gmail.com
Remote:

Profile Name: GitFlick
Service_User: GitFlick
User Name: Itdobro
Email: itsdobro@gmail.com
Remote:

Profile Name: GitHub_Avdushin
Service_User: GitHub_Avdushin
User Name: Avdushin
Email: avdushinbeaver1@gmail.com
Remote:
```

</details>

<details>
<summary>Диагностика конфигураций (gitpm doctor)</summary>

```bash
$ gitpm doctor
Configuration directory: C:\Users\avdus\AppData\Roaming\GitProjectManager\gitpm\config
Configuration file: C:\Users\avdus\AppData\Roaming\GitProjectManager\gitpm\config/profiles.json
```

</details>

<details>
<summary>Удаление профиля (gitpm remove)</summary>

```bash
$ gitpm remove
Select a profile to remove:
1. GitLab_Avdushin
2. Example_profile
3. GitFlick
4. GitHub_Avdushin
Enter the number of the profile: 2
Are you sure you want to remove the profile 'Example_profile' (y/n): y
Profile 'Example_profile' removed successfully!
```

</details>

<details>
<summary>Удаление всех профилей (gitpm remove-all)</summary>

```bash
$ gitpm remove-all
Are you sure you want to remove all profiles? (y/n): y
All profiles have been removed successfully!
```

</details>


## Как установить

### Требования

- Git (необходимо для работы профилей)
- Rust (требуется для компиляции, если вы собираетесь собрать программу из исходного кода)

### Установка

1. **Скачайте нужный файл**:

   - **Windows**: [Скачать `gitpm.exe`](bin/windows/gitpm.exe)
   - **Linux**: [Скачать `gitpm`](bin/linux/gitpm)
   - **macOS**: [Скачать `gitpm`](bin/macos/gitpm)

2. **Разместите файл в удобной директории**:
   Например, создайте директорию `~/gitpm` и поместите туда файл `gitpm` или `gitpm.exe`.

3. **Добавьте директорию в PATH**:

   #### Windows

   - Перейдите в `Настройки > Система > О системе > Дополнительные параметры системы`.
   - В разделе "Переменные среды" найдите переменную `Path` и нажмите "Изменить".
   - Добавьте путь до директории `gitpm`, например: `C:\Users\<Ваше_Имя>\gitpm`.
   - Сохраните изменения и перезапустите терминал.

   #### Linux и macOS

   ```bash
   https://github.com/Avdushin/gitpm
   cd gitpm
   # linux
   sudo cp -rf bin/linux/gitpm /usr/local/bin
   # macos
   sudo cp -rf bin/macos/gitpm /usr/local/bin
   ```
   

4. **Проверьте установку**:
   Откройте новый терминал и выполните:
   ```bash
   gitpm --help
   ```
   Если программа установлена корректно, отобразится справка по командам.

### Установка из исходного кода

Если вы хотите собрать Git Profile Manager из исходников:

1. Установите [Rust](https://www.rust-lang.org/tools/install).
2. Клонируйте репозиторий:
   ```bash
   git clone https://github.com/Avdushin/gitpm
   cd gitpm
   ```
3. Соберите проект для вашей системы:
   ```bash
   cargo build --release
   ```
4. Переместите скомпилированный файл из `target/release/` в директорию `~/gitpm` и добавьте ее в PATH (описано выше).
