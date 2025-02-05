use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859525: FileFormat = FileFormat {
    id: 105_859_525,
    source_type: SourceType::Wikidata,
    name: "VLBI Experiment (with rem)",
    extensions: &["vex"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
