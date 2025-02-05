use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129828012: FileFormat = FileFormat {
    id: 129_828_012,
    source_type: SourceType::Wikidata,
    name: "Ioke source code file",
    extensions: &["ik"],
    media_types: &["text/x-iokesrc"],
    signatures: &[],
    related_formats: &[],
};
