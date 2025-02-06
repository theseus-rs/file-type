use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_52060319: FileFormat = FileFormat {
    id: 52_060_319,
    source_type: SourceType::Wikidata,
    name: "JustWrite Text Document",
    extensions: &["jw", "jwt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
