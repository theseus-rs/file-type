use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889366: FileFormat = FileFormat {
    id: 114_889_366,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Tote Bag file",
    extensions: &["stb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
