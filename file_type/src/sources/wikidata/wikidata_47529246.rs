use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47529246: FileFormat = FileFormat {
    id: 47_529_246,
    source_type: SourceType::Wikidata,
    name: "SuperScape Virtual Reality Format",
    extensions: &["svr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
