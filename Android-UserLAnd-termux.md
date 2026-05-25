
# Instrukcja konfiguracji lokalnej skrzynki Rust (UserLAnd + Ubuntu)

Mój kompletny przewodnik krok po kroku – od instalacji aplikacji ze sklepu Google Play do odpalenia własnego kodu Rust na telefonie całkowicie offline (bez internetu).

---

## 1. Instalacja i pierwsze uruchomienie z Google Play

1. Otwieram sklep **Google Play** na telefonie.

2. Wyszukuję aplikację **UserLAnd** (autor: UserLAnd Technologies) i klikam **Zainstaluj**.

3. Po uruchomieniu aplikacji widzę listę dostępnych systemów. Klikam na **Ubuntu**.

4. Wyskakuje okno konfiguracji:
   - W polu **Username** wpisuję swoją nazwę użytkownika (np. `root` lub cokolwiek innego).
   - W polu **Password** wpisuję swoje hasło dostępowe, które muszę zapamiętać.
   - W polu **VNC Password** wpisuję dowolne hasło zabezpieczające.

5. W okienku **Choose Connection Type** wybieram opcję **Minimal**.

6. W kolejnym okienku zaznaczam **Terminal** oraz ptaszek **Always use this setting**, po czym klikam **KONTYNUUJ**.

7. Aplikacja pobiera system z internetu. Po zakończeniu otwiera się czarna konsola tekstowa z systemem Ubuntu.

---

## 2. Przygotowanie skrzynki i instalacja narzędzi (Wymaga internetu)

Domyślny system jest całkowicie pusty. Aby przygotować go do programowania, wklejam w konsoli poniższą komendę i zatwierdzam Enterem:

```bash
sudo apt update && sudo apt install -y curl build-essential git
```

*Uwaga: Gdy system zapyta o hasło ([sudo] password for ...), wpisuję moje hasło zdefiniowane podczas instalacji w punkcie 1.*

## 3. Instalacja i konfiguracja Rusta 1.96 (Beta)
Wklejam do konsoli oficjalny instalator środowiska Rust, który automatycznie wybierze wersję rozwojową (beta):

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh -s -- --default-toolchain beta -y
```

Po zakończeniu instalacji aktywuję menedżer paczek Cargo w obecnej sesji konsoli:

```bash
. "$HOME/.cargo/env"
```

Następnie wymuszam instalację wersji 1.96, ignorując brakujące na serwerach opcjonalne dodatki narzędziowe, i ustawiam ją jako domyślny kompilator:

```bash
rustup toolchain install beta --allow-downgrade && rustup default 1.96.0-beta
```
Sprawdzam, czy system poprawnie widzi narzędzia:

```bash
cargo --version
```
Konsola musi zwrócić informację o wersji: cargo 1.96.0-beta.

## 4. Klonowanie kodu i budowanie pamięci podręcznej (Cache)

Póki mam jeszcze dostęp do sieci, muszę sklonować moje repozytorium z kodem z GitHuba i zmusić system do pobrania wszystkich zewnętrznych bibliotek na dysk telefonu.

### Krok A: Pobranie kodu (Klonowanie)

```bash
git clone [https://github.com/j-Cis/tmp-nums.git](https://github.com/j-Cis/tmp-nums.git
```

### Krok B: Wejście do folderu projektu

```bash
cd tmp-nums
```

### Krok C: Budowanie cache do pracy offline

```bash
cargo check
```

Ta komenda pobiera z internetu wszystkie zależności z pliku Cargo.toml i zapisuje je w pamięci telefonu. Od tej sekundy internet nie jest mi już do niczego potrzebny.
## 5. Praca z kodem w trybie samolotowym (Offline)
Gdy nie mam połączenia z siecią, a chcę rozwijać i testować mój system CAS:
 1. Otwieram aplikację **UserLAnd** i klikam moją sesję **Ubuntu**.
 2. Od razu przechodzę do folderu mojego projektu:

   ```bash
   cd ~/tmp-nums
   ```

 3. Mogę swobodnie sprawdzać składnię oraz uruchamiać program bezpośrednio na procesorze telefonu:

   ```bash
   cargo check
   cargo run
   ```



