use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62625561: FileFormat = FileFormat {
    id: 62_625_561,
    source_type: SourceType::Wikidata,
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
    signatures: &[],
    related_formats: &[],
};
