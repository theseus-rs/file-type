use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131322036: FileFormat = FileFormat {
    id: 131_322_036,
    puid: "wikidata/131322036",
    name: "Treetop file format",
    extensions: &["treetop", "tt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
