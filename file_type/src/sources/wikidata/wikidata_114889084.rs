use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114889084: FileFormat = FileFormat {
    id: 114_889_084,
    source_type: SourceType::Wikidata,
    name: "Scrapbook Factory Deluxe Mousepad file",
    extensions: &["sms"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
