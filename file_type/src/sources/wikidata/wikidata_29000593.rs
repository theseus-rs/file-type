use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000593: FileFormat = FileFormat {
    id: 29_000_593,
    source_type: SourceType::Wikidata,
    name: "HFS Plus Journal",
    extensions: &["journal"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
