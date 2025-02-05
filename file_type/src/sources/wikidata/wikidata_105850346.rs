use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850346: FileFormat = FileFormat {
    id: 105_850_346,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM mCrypt encrypted (v0.1b)",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
