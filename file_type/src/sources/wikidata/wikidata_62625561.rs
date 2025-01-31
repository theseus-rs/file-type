use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62625561: FileFormat = FileFormat {
    id: 62_625_561,
    puid: "wikidata/62625561",
    name: "Bash script",
    extensions: &[
        "bash",
        "bash_profile",
        "bashrc",
        "bsh",
        "csh",
        "profile",
        "sh",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
