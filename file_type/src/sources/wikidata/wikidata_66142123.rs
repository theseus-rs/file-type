use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66142123: FileFormat = FileFormat {
    id: 66_142_123,
    source_type: SourceType::Wikidata,
    name: "ACCDE file format",
    extensions: &["accde"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
