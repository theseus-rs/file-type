use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849583: FileFormat = FileFormat {
    id: 105_849_583,
    source_type: SourceType::Wikidata,
    name: "16bit DOS COM Crack Soft's cryptor encrypted",
    extensions: &["com"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
