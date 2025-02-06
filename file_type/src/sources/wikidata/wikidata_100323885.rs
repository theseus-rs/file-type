use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100323885: FileFormat = FileFormat {
    id: 100_323_885,
    source_type: SourceType::Wikidata,
    name: "Corel Gallery Clipart",
    extensions: &["bmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
