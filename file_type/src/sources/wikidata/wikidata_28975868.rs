use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975868: FileFormat = FileFormat {
    id: 28_975_868,
    puid: "wikidata/28975868",
    name: "OOGL SPHERE file",
    extensions: &["sph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
