use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889732: FileFormat = FileFormat {
    id: 114_889_732,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Cover file",
    extensions: &["ssc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
