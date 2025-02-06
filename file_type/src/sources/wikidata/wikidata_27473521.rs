use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473521: FileFormat = FileFormat {
    id: 27_473_521,
    source_type: SourceType::Wikidata,
    name: "Advanced Forensic Format, version 2.0",
    extensions: &["aff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
