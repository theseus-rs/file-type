use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_39069698: FileFormat = FileFormat {
    id: 39_069_698,
    source_type: SourceType::Wikidata,
    name: "Ion",
    extensions: &["ion"],
    media_types: &["application/ion"],
    signatures: &[],
    related_formats: &[],
};
