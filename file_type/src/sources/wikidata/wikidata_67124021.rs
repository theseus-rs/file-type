use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67124021: FileFormat = FileFormat {
    id: 67_124_021,
    source_type: SourceType::Wikidata,
    name: "Print Artist greeting card file format",
    extensions: &["gc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
