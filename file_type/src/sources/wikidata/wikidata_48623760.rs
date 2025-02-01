use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48623760: FileFormat = FileFormat {
    id: 48_623_760,
    puid: "wikidata/48623760",
    name: "Paint Shop Pro Image, version 5",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
