use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959881: FileFormat = FileFormat {
    id: 27_959_881,
    source_type: SourceType::Wikidata,
    name: "Propellerhead Reason NN-XT Patch File",
    extensions: &["sx2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
