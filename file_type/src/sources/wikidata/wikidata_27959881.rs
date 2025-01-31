use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959881: FileFormat = FileFormat {
    id: 27_959_881,
    puid: "wikidata/27959881",
    name: "Propellerhead Reason NN-XT Patch File",
    extensions: &["sx2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
