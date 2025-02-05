use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000735: FileFormat = FileFormat {
    id: 29_000_735,
    source_type: SourceType::Wikidata,
    name: "VOL",
    extensions: &["vol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
