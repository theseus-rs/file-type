use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48695244: FileFormat = FileFormat {
    id: 48_695_244,
    source_type: SourceType::Wikidata,
    name: "DEC Data Exchange File",
    extensions: &["dx"],
    media_types: &["application/dec-dx"],
    internal_signatures: &[],
    related_formats: &[],
};
