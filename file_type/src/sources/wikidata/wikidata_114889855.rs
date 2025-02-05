use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889855: FileFormat = FileFormat {
    id: 114_889_855,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Effects Category file",
    extensions: &["tlx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
