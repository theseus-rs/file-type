use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109596469: FileFormat = FileFormat {
    id: 109_596_469,
    puid: "wikidata/109596469",
    name: "DrawPlus Template",
    extensions: &["dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
