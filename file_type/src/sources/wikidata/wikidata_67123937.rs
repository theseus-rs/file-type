use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67123937: FileFormat = FileFormat {
    id: 67_123_937,
    source_type: SourceType::Wikidata,
    name: "Print Artist business card file format",
    extensions: &["bc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
