use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72961170: FileFormat = FileFormat {
    id: 72_961_170,
    source_type: SourceType::Wikidata,
    name: "Prescription Drug Event format",
    extensions: &["pde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
