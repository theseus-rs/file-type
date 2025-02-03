use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_4037242: FileFormat = FileFormat {
    id: 4_037_242,
    source_type: SourceType::Wikidata,
    name: "Desktop.ini",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
