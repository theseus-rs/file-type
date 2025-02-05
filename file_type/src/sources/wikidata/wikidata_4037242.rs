use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4037242: FileFormat = FileFormat {
    id: 4_037_242,
    source_type: SourceType::Wikidata,
    name: "Desktop.ini",
    extensions: &["ini"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
