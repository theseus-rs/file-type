use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84996757: FileFormat = FileFormat {
    id: 84_996_757,
    source_type: SourceType::Wikidata,
    name: "HP Photo Album",
    extensions: &["albm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
