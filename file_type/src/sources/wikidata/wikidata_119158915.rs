use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119158915: FileFormat = FileFormat {
    id: 119_158_915,
    source_type: SourceType::Wikidata,
    name: "Digital Image PNG Plus",
    extensions: &["png"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
