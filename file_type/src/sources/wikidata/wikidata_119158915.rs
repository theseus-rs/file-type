use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119158915: FileFormat = FileFormat {
    id: 119_158_915,
    source_type: SourceType::Wikidata,
    name: "Digital Image PNG Plus",
    extensions: &["png"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
