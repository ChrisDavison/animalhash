# animalhash

Generate a docker-esque adjective-colour-animal keyword.

Can optionally disable the 'animal', 'adjective', and 'colour' portions, as well
as semi-titlecase the result.

Examples:

    animalhash => smalltealtortoise
    animalhash -t => affectionateMagentaPuffin
    animalhash --no-animal => bigred
    animalhash --no-adjective => redoctopus
    animalhash --no-colou => reverantmeadowlark

## Usage

    animalhash [OPTIONS]

    Options:
        -l --lowercase   Don't semi-titlecase the output
        --no-colour      Don't use a colour
        --no-adjective   Don't use an adjective
        --no-animal      Don't use an animal
