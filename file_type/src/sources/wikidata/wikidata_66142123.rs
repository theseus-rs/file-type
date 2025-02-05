use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66142123: FileFormat = FileFormat {
    id: 66_142_123,
    source_type: SourceType::Wikidata,
    name: "ACCDE file format",
    extensions: &["accde"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
