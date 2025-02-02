use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111341513: FileFormat = FileFormat {
    id: 111_341_513,
    source_type: SourceType::Wikidata,
    name: "Signed byte (8-bit) data",
    extensions: &["sb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
