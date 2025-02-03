use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889069: FileFormat = FileFormat {
    id: 114_889_069,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Apron file",
    extensions: &["sap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
