use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650344: FileFormat = FileFormat {
    id: 29_650_344,
    source_type: SourceType::Wikidata,
    name: "PAQ",
    extensions: &["pa6", "pa7", "pa8", "paq8p"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
