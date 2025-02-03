use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111341823: FileFormat = FileFormat {
    id: 111_341_823,
    source_type: SourceType::Wikidata,
    name: "Signed dwords (32-bit) data",
    extensions: &["sdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
