use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967124: FileFormat = FileFormat {
    id: 27_967_124,
    source_type: SourceType::Wikidata,
    name: "CM3",
    extensions: &["cm3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
