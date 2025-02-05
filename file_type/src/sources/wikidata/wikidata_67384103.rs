use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67384103: FileFormat = FileFormat {
    id: 67_384_103,
    source_type: SourceType::Wikidata,
    name: "ArtMoney Table File",
    extensions: &["amt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
