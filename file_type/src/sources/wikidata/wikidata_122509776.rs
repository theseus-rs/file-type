use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122509776: FileFormat = FileFormat {
    id: 122_509_776,
    source_type: SourceType::Wikidata,
    name: "Pretty Good Privacy public key ring data file",
    extensions: &["pubkr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
