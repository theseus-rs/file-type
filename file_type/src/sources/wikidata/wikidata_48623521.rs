use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48623521: FileFormat = FileFormat {
    id: 48_623_521,
    puid: "wikidata/48623521",
    name: "Paint Shop Pro Image, version 3",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
