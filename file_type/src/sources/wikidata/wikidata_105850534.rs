use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850534: FileFormat = FileFormat {
    id: 105_850_534,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Ryptor encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
