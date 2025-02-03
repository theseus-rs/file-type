use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650344: FileFormat = FileFormat {
    id: 29_650_344,
    source_type: SourceType::Wikidata,
    name: "PAQ",
    extensions: &["pa6", "pa7", "pa8", "paq8p"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
