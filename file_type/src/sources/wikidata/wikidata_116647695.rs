use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116647695: FileFormat = FileFormat {
    id: 116_647_695,
    puid: "wikidata/116647695",
    name: "KeyForm 4.0 Document",
    extensions: &["kfm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
