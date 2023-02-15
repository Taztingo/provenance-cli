# provenance-script
A script manager to improve Provenance user experience.

# Usage

# Examples

# Autocomplete

The autocomplete feature is one of the big differences that separates provenance-script from provenanced cli. It allows
developers to blaze through the different commands. The following must be done to enable it:

1. Generate the autocomplete script and redirect it to a provenance-script.bash file.
`provenance-script autocomplete > provenance-script.bash`

2. Next, copy it to the bash-completions folder. This may require sudo.
`sudo mv provenance-script.bash /usr/share/bash-completion/completions/provenance-script.bash`

3. Make sure your binary is accessible from PATH. This may require sudo.
`cp provenance-script /usr/local/bin`

4. Restart bash
`exec $0`