use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663160: FileFormat = FileFormat {
    id: 66_663_160,
    source_type: SourceType::Wikidata,
    name: "eSuite word processor format",
    extensions: &["html-wp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
