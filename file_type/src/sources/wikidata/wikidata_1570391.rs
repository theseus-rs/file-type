use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1570391: FileFormat = FileFormat {
    id: 1_570_391,
    source_type: SourceType::Wikidata,
    name: "Uuencoding",
    extensions: &["uu", "uue"],
    media_types: &["text/x-uuencode"],
    signatures: &[],
    related_formats: &[],
};
