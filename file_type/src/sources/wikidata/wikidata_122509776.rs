use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122509776: FileFormat = FileFormat {
    id: 122_509_776,
    source_type: SourceType::Wikidata,
    name: "Pretty Good Privacy public key ring data file",
    extensions: &["pubkr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
