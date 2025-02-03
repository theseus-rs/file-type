use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25345930: FileFormat = FileFormat {
    id: 25_345_930,
    source_type: SourceType::Wikidata,
    name: "Citrine",
    extensions: &["ctr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
