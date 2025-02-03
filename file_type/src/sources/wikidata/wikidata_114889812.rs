use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114889812: FileFormat = FileFormat {
    id: 114_889_812,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Caledar file",
    extensions: &["scl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
