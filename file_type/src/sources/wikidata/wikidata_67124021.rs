use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67124021: FileFormat = FileFormat {
    id: 67_124_021,
    source_type: SourceType::Wikidata,
    name: "Print Artist greeting card file format",
    extensions: &["gc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
