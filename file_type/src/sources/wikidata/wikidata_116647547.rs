use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116647547: FileFormat = FileFormat {
    id: 116_647_547,
    puid: "wikidata/116647547",
    name: "Form file",
    extensions: &["ofm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
