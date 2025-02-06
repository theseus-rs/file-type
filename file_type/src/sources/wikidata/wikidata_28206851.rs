use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206851: FileFormat = FileFormat {
    id: 28_206_851,
    source_type: SourceType::Wikidata,
    name: "Secure Pick Ax file",
    extensions: &["pax"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
