use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28449455: FileFormat = FileFormat {
    id: 28_449_455,
    source_type: SourceType::Wikidata,
    name: "TOML",
    extensions: &["toml"],
    media_types: &["application/toml"],
    signatures: &[],
    related_formats: &[],
};
