use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51918148: FileFormat = FileFormat {
    id: 51_918_148,
    source_type: SourceType::Wikidata,
    name: "XYWrite Document",
    extensions: &["xy"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
