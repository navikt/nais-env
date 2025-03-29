# nais-env

## Om prosjektet

`nais-env` er CLI et verktøy for å hente konfigurasjonsvariabler fra NAIS  og gjøre dem tilgjengelige lokalt. Dette forenkler lokal utvikling ved å gi deg muligheten til å jobbe med de samme miljøvariablene som finnes i Kubernetes.


## Funksjonalitet

- Henter miljøvariabler og hemmeligheter fra NAIS-konfigurasjonen
- Kan lagre disse til en fil for senere bruk
- Kan starte et nytt shell med alle miljøvariabler satt
- Mulighet for å skrive ut hemmelighetene direkte (når det er trygt å gjøre det)

## Installasjon

TODO: Fix etter github-action

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
```

## Forutsetninger

- Du må være autentisert mot Kubernetes-klusteret
- nais.yaml-filen må være korrekt konfigurert

## Bidrag

Bidrag er hjertelig velkommen! Vennligst send inn en pull request eller opprett et issue hvis du har forslag til forbedringer.
