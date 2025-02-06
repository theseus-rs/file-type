use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473397: FileFormat = FileFormat {
    id: 27_473_397,
    source_type: SourceType::Wikidata,
    name: "Advanced Forensic Format, version 1.0",
    extensions: &["aff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
