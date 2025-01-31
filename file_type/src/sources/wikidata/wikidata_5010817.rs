use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5010817: FileFormat = FileFormat {
    id: 5_010_817,
    puid: "wikidata/5010817",
    name: "CFS",
    extensions: &["cfs"],
    media_types: &["application/x-cfs-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
