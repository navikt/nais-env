# nais-env

## Om prosjektet

`nais-env` er CLI et verktøy for å hente konfigurasjonsvariabler fra NAIS og gjøre dem tilgjengelige lokalt. Dette forenkler lokal utvikling ved å gi deg muligheten til å jobbe med de samme miljøvariablene som finnes i Kubernetes.


## Funksjonalitet

- Henter miljøvariabler og hemmeligheter fra NAIS-konfigurasjonen
- Kan lagre disse til en fil for senere bruk
- Kan starte et nytt shell med alle miljøvariabler satt
- Mulighet for å skrive ut hemmelighetene direkte (når det er trygt å gjøre det)
- Legger automatisk til genererte filer i `.git/info/exclude` for å unngå at sensitive data sjekkes inn
- Kan rydde opp og slette alle genererte miljøfiler med `--clear-files`
- Setter miljøvariabelen `NAIS_ENV_ACTIVE=true` når shell startes med `--shell`
- Støtter spesifisering av Kubernetes-kontekst (begrenset til 'nais-dev' og 'dev-fss')

## Installasjon

TODO: Fix etter github-action

### Tab-completion

Nais-env støtter tab-completion for bash, zsh, fish og PowerShell. Du kan generere completion-skript ved å bruke følgende kommandoer:

#### Bash

```bash
# Generer og installer completion-skript for gjeldende bruker
nais-env completion bash > ~/.bash_completion

# På noen systemer må du kanskje laste inn skriptet manuelt
echo 'source ~/.bash_completion' >> ~/.bashrc
```

#### Zsh

```zsh
# Opprett completion-katalog hvis den ikke finnes
mkdir -p ~/.zsh/completion

# Generer completion-skript
nais-env completion zsh > ~/.zsh/completion/_nais-env

# Legg til i .zshrc hvis ikke allerede inkludert
echo 'fpath=(~/.zsh/completion $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

#### Fish

```fish
# Generer og installer completion-skript
nais-env completion fish > ~/.config/fish/completions/nais-env.fish
```

#### PowerShell

```powershell
# Generer completion-skript
nais-env completion powershell > nais-env.ps1

# Legg til i PowerShell-profilen
echo ". /path/to/nais-env.ps1" >> $PROFILE
```

Shell må restartes for at tab-completion skal fungere

## Bruk

```bash
# Vis hjelp
nais-env --help

# Hent miljøvariabler og lagre til fil
nais-env --config path/to/nais.yaml --file .env

# Start et shell med alle miljøvariablene tilgjengelig
nais-env --config path/to/nais.yaml --shell

# Vis alle miljøvariablene i terminalen
nais-env --config path/to/nais.yaml --print

# Slett alle miljøfiler som er opprettet av nais-env
nais-env --clear-files

# Spesifiser Kubernetes-kontekst (nais-dev eller dev-fss)
nais-env --config path/to/nais.yaml --context dev-fss
```

### Tilpasning av zsh-prompt

For å få en tilpasset prompt i zsh når du bruker `--shell`, kan du legge til følgende i din `.zshrc`:

```zsh
if [[ -n "$NAIS_ENV_ACTIVE" ]]; then
  PROMPT="%F{green}[NAIS-ENV:$NAIS_ENV_CONFIG]%f %~ $ "
fi
```

Dette vil gi deg en tydelig indikasjon når du jobber i et shell med NAIS-miljøvariabler.

## Forutsetninger

- Du må være autentisert mot Kubernetes-klusteret
- nais.yaml-filen må være korrekt konfigurert

## Bidrag

Bidrag er hjertelig velkommen! Vennligst send inn en pull request eller opprett et issue hvis du har forslag til forbedringer.
