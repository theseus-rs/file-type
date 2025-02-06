use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131144603: FileFormat = FileFormat {
    id: 131_144_603,
    source_type: SourceType::Wikidata,
    name: "Sophia file format",
    extensions: &["aes"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
