use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_13422998: FileFormat = FileFormat {
    id: 13_422_998,
    source_type: SourceType::Wikidata,
    name: "HTTP Archive format",
    extensions: &["har"],
    media_types: &["text/json"],
    signatures: &[],
    related_formats: &[],
};
