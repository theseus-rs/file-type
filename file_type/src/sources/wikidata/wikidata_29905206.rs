use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905206: FileFormat = FileFormat {
    id: 29_905_206,
    source_type: SourceType::Wikidata,
    name: "Self-Dissolving Archive",
    extensions: &["sda"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
