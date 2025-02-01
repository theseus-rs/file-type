use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342780: FileFormat = FileFormat {
    id: 111_342_780,
    puid: "wikidata/111342780",
    name: "Propellerheads Reason 2 NN-XT format",
    extensions: &["sxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
