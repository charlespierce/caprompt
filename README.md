# caprompt
Simple shell prompt formatter. See below for scripts to customize prompt:

## Unix Shells (Bash / Zsh)

Add the following to your profile script (e.g. `.bashrc`)
```bash
PS1="$(caprompt $?)"
```

## Powershell

Add this to the Profile script (found from `$Profile` in the terminal)
```powershell
function prompt {
    $previousEncoding = [Console]::OutputEncoding
    [Console]::OutputEncoding = [Text.Encoding]::UTF8

    $output = Invoke-Expression "caprompt $?"
    Set-PSReadLineOption -ExtraPromptLineCount (($output | Measure-Object -Line).Lines - 1)
    $output -join "`n"

    [Console]::OutputEncoding = $previousEncoding
}
```