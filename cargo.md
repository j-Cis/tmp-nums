
# Ściągawka z komend Cargo i pracy Offline

Mój podręczny zestaw poleceń do zarządzania projektem w Ruście za pomocą Cargo, ze szczególnym uwzględnieniem przygotowania środowiska do pracy bez internetu.

---

## 1. Zabezpieczenie paczek do pracy 100% Offline

Gdy dopisuję nowe zależności (dependencies) w pliku `Cargo.toml`, muszę je pobrać na dysk telefonu, zanim odetnę internet. Mam na to dwa sposoby:

### Sposób A: Automatyczny Cache (Najprostszy)

Gdy mam internet, wchodzę do folderu projektu i odpalam:
```bash
cargo check
```

*Jak to działa:* Cargo skanuje Cargo.toml, pobiera brakujące paczki z repozytorium crates.io i zapisuje je w globalnym folderze .cargo na telefonie. Gdy wejdę w tryb samolotowy, Cargo automatycznie wyciągnie je z tego lokalnego cache'u.
### Sposób B: Vendorowanie (Pełna niezależność projektu)
Jeśli chcę, aby absolutnie wszystkie paczki były fizycznie skopiowane bezpośrednio do folderu mojego projektu (co gwarantuje, że projekt zbuduje się na dowolnym urządzeniu bez dostępu do sieci), używam:

```bash
cargo vendor
```
*Jak to działa:* Ta komenda tworzy w projekcie katalog vendor/ i pobiera tam kody źródłowe wszystkich bibliotek. Po jej wykonaniu, Cargo podpowiada 3 linijki kodu konfiguracji, które należy wkleić do pliku .cargo/config.toml, aby system zawsze szukał paczek na dysku zamiast w sieci.

## 2. Podstawowe komendy deweloperskie

Te polecenia wykonuję lokalnie w terminalu UserLAnd podczas pisania kodu:
 * **Szybkie sprawdzenie składni (bez pełnej kompilacji):**
   ```bash
   cargo check
   
   ```
   *Używam tego non-stop podczas pisania, żeby upewnić się, że nie zrobiłem błędów w typach lub pamięci (Ownership).*
 * **Kompilacja i natychmiastowe uruchomienie programu:**

   ```bash
   cargo run
   ```
   *Kompiluje plik wykonywalny i od razu go odpala, wypluwając wyniki testów CAS w konsoli.*
 * **Zbudowanie projektu w wersji testowej (Debug):**

   ```bash
   cargo build
   ```
   *Tworzy plik binarny w katalogu target/debug/.*
## 3. Czyszczenie i optymalizacja
 * **Czyszczenie plików tymczasowych:**

   ```bash
   cargo clean
   ```
   *Usuwa cały katalog target/, w którym zbierają się skompilowane pliki. Warto to zrobić, jeśli na telefonie zaczyna brakować miejsca na dysku (pamiętaj, że kolejny cargo run po czyszczeniu potrwa dłużej, bo system buduje wszystko od zera).*
 * **Budowanie ostatecznej, super-szybkiej wersji (Release):**

   ```bash
   cargo build --release
   ```

   *Uruchamia pełną optymalizację kodu przez kompilator. Program działa wtedy najszybciej jak to możliwe na procesorze telefonu, ale sama kompilacja trwa znacznie dłużej.*
