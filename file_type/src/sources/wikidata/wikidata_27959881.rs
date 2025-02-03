use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27959881: FileFormat = FileFormat {
    id: 27_959_881,
    source_type: SourceType::Wikidata,
    name: "Propellerhead Reason NN-XT Patch File",
    extensions: &["sx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
