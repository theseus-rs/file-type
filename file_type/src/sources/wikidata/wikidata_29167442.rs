use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167442: FileFormat = FileFormat {
    id: 29_167_442,
    source_type: SourceType::Wikidata,
    name: "OFIP",
    extensions: &["ofip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
