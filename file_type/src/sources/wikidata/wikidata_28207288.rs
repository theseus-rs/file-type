use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207288: FileFormat = FileFormat {
    id: 28_207_288,
    source_type: SourceType::Wikidata,
    name: "Slow-scan television",
    extensions: &["hrz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
