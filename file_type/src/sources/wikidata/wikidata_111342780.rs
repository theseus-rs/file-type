use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342780: FileFormat = FileFormat {
    id: 111_342_780,
    source_type: SourceType::Wikidata,
    name: "Propellerheads Reason 2 NN-XT format",
    extensions: &["sxt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
