use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116647547: FileFormat = FileFormat {
    id: 116_647_547,
    source_type: SourceType::Wikidata,
    name: "Form file",
    extensions: &["ofm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
