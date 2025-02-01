use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47529246: FileFormat = FileFormat {
    id: 47_529_246,
    puid: "wikidata/47529246",
    name: "SuperScape Virtual Reality Format",
    extensions: &["svr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
