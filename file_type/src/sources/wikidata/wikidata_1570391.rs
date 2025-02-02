use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1570391: FileFormat = FileFormat {
    id: 1_570_391,
    source_type: SourceType::Wikidata,
    name: "Uuencoding",
    extensions: &["uu", "uue"],
    media_types: &["text/x-uuencode"],
    internal_signatures: &[],
    related_formats: &[],
};
