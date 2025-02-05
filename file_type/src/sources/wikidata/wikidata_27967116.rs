use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967116: FileFormat = FileFormat {
    id: 27_967_116,
    source_type: SourceType::Wikidata,
    name: "ASC Sound Master module",
    extensions: &["asc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
