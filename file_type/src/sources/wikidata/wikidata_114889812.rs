use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889812: FileFormat = FileFormat {
    id: 114_889_812,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Caledar file",
    extensions: &["scl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
