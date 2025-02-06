use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108328831: FileFormat = FileFormat {
    id: 108_328_831,
    source_type: SourceType::Wikidata,
    name: "Universe Sandbox Data File",
    extensions: &["ubox"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
