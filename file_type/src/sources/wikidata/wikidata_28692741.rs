use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28692741: FileFormat = FileFormat {
    id: 28_692_741,
    puid: "wikidata/28692741",
    name: "FAV File Format",
    extensions: &["fav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
