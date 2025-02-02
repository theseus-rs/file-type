use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889855: FileFormat = FileFormat {
    id: 114_889_855,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Effects Category file",
    extensions: &["tlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
