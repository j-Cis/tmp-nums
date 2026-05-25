
# Konfiguracja Termux -> GitHub Codespaces (Dedykowany Przewodnik)

Szybka ściągawka do zarządzania i łączenia z dedykowanym środowiskiem deweloperskim.

---

## 1. Wybór uprawnień dla Tokenu (Classic)
Podczas generowania tokenu na stronie GitHub (Settings -> Developer settings -> Personal access tokens -> Tokens classic) musisz zaznaczyć wyłącznie te uprawnienia:
* **`repo`** (pełny dostęp do kodu)
* **`codespace`** (pełny dostęp do maszyn)
* **`read:org`** (weryfikacja profilu)

---

## 2. Logowanie w Termuxie
Wywołaj komendę:
```bash
gh auth login
```

Wybierz kolejno: GitHub.com -> SSH -> Paste an authentication token -> Wklej wygenerowany token.

## 3. Procedura Naprawcza Tunelu ("Closed Network Connection")

Gdy połączenie z maszyną zostanie przerwane lub zablokuje porty, wykonaj poniższe kroki.

### Krok A: Wymuszenie zatrzymania (Stop)

Ogólny wzorzec: gh codespace stop -c NAZWA_MASZYNY

**Komenda dla mojej maszyny:**
```bash
gh codespace stop -c sturdy-orbit-vpq6x5rp946h5qg
```

### Krok B: Odczekaj 10 sekund

### Krok C: Połączenie bezpośrednie (SSH)

Ogólny wzorzec: gh codespace ssh -c NAZWA_MASZYNY

**Komenda dla mojej maszyny:**

```bash
gh codespace ssh -c sturdy-orbit-vpq6x5rp946h5qg
```

## 4. Praca z kodem w locie (Aplikacja GitHub + Termux)

 1. Edytuj kod w natywnej aplikacji GitHub na telefonie i robisz Commit.

 2. Wchodzisz do Termuxa (musisz być połączony przez SSH z maszyną).

 3. **Zawsze przed uruchomieniem testów pobierz zmiany z repozytorium:**

   ```bash
   git pull
   
   ```

## 5. Środowisko Rusta 1.96 (Beta) na nowej maszynie

Jeśli po połączeniu maszyna nie widzi komendy cargo, odpal:

```bash
. "$HOME/.cargo/env"
```

Instalacja wersji 1.96 z pominięciem brakujących komponentów w kanale beta:

```bash
rustup toolchain install beta --allow-downgrade
```

Ustawienie jako domyślny kompilator:

```bash
rustup default 1.96.0-beta
```

Sprawdzenie i uruchomienie:

```bash
cargo --version
cargo check
cargo run
```

```
