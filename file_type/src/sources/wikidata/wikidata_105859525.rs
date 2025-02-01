use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859525: FileFormat = FileFormat {
    id: 105_859_525,
    puid: "wikidata/105859525",
    name: "VLBI Experiment (with rem)",
    extensions: &["vex"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
