use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72959001: FileFormat = FileFormat {
    id: 72_959_001,
    source_type: SourceType::Wikidata,
    name: "PrintArtist project",
    extensions: &["pa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
