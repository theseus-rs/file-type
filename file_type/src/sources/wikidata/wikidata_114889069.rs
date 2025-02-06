use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889069: FileFormat = FileFormat {
    id: 114_889_069,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Apron file",
    extensions: &["sap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
