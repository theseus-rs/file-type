use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28449455: FileFormat = FileFormat {
    id: 28_449_455,
    source_type: SourceType::Wikidata,
    name: "TOML",
    extensions: &["toml"],
    media_types: &["application/toml"],
    internal_signatures: &[],
    related_formats: &[],
};
