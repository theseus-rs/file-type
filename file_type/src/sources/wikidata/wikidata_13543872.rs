use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_13543872: FileFormat = FileFormat {
    id: 13_543_872,
    source_type: SourceType::Wikidata,
    name: "Wii ISO Archive",
    extensions: &["wbfs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
