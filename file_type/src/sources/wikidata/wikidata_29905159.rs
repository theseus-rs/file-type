use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905159: FileFormat = FileFormat {
    id: 29_905_159,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System transport file",
    extensions: &["stx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
