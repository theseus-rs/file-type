use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72959001: FileFormat = FileFormat {
    id: 72_959_001,
    source_type: SourceType::Wikidata,
    name: "PrintArtist project",
    extensions: &["pa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
