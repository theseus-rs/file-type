use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58800062: FileFormat = FileFormat {
    id: 58_800_062,
    source_type: SourceType::Wikidata,
    name: "PowerProject",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
