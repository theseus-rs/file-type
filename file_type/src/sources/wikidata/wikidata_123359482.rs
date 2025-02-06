use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123359482: FileFormat = FileFormat {
    id: 123_359_482,
    source_type: SourceType::Wikidata,
    name: "Personal History Project",
    extensions: &["phst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
