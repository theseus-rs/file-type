use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850251: FileFormat = FileFormat {
    id: 105_850_251,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM The WiZ Cryptor encrypted (v1.00a)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
