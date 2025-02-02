use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_6505523: FileFormat = FileFormat {
    id: 6_505_523,
    source_type: SourceType::Wikidata,
    name: "Layered Image File Format",
    extensions: &["liff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
