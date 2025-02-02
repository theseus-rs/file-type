use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859525: FileFormat = FileFormat {
    id: 105_859_525,
    source_type: SourceType::Wikidata,
    name: "VLBI Experiment (with rem)",
    extensions: &["vex"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
