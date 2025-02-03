use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72961170: FileFormat = FileFormat {
    id: 72_961_170,
    source_type: SourceType::Wikidata,
    name: "Prescription Drug Event format",
    extensions: &["pde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
