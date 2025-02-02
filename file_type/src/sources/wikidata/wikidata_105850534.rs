use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850534: FileFormat = FileFormat {
    id: 105_850_534,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Ryptor encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
