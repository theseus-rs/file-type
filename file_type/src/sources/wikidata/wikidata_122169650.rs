use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122169650: FileFormat = FileFormat {
    id: 122_169_650,
    source_type: SourceType::Wikidata,
    name: "Password Cache File",
    extensions: &["epc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
