use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6717908: FileFormat = FileFormat {
    id: 6_717_908,
    puid: "wikidata/6717908",
    name: "MSSTYLES",
    extensions: &["msstyles"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
