use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129828012: FileFormat = FileFormat {
    id: 129_828_012,
    puid: "wikidata/129828012",
    name: "Ioke source code file",
    extensions: &["ik"],
    media_types: &["text/x-iokesrc"],
    internal_signatures: &[],
    related_formats: &[],
};
