use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117835557: FileFormat = FileFormat {
    id: 117_835_557,
    puid: "wikidata/117835557",
    name: "Knowledge Access file",
    extensions: &["cpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
