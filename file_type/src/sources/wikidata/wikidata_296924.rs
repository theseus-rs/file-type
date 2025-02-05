use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_296924: FileFormat = FileFormat {
    id: 296_924,
    source_type: SourceType::Wikidata,
    name: "ART image file format",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
