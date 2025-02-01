use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757992: FileFormat = FileFormat {
    id: 28_757_992,
    puid: "wikidata/28757992",
    name: "ISZ",
    extensions: &["isz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
