use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129828012: FileFormat = FileFormat {
    id: 129_828_012,
    source_type: SourceType::Wikidata,
    name: "Ioke source code file",
    extensions: &["ik"],
    media_types: &["text/x-iokesrc"],
    internal_signatures: &[],
    related_formats: &[],
};
