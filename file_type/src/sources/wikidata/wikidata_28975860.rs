use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975860: FileFormat = FileFormat {
    id: 28_975_860,
    puid: "wikidata/28975860",
    name: "OOGL MESH file",
    extensions: &["mesh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
