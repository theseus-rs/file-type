use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342780: FileFormat = FileFormat {
    id: 111_342_780,
    source_type: SourceType::Wikidata,
    name: "Propellerheads Reason 2 NN-XT format",
    extensions: &["sxt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
