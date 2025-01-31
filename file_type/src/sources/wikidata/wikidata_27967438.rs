use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967438: FileFormat = FileFormat {
    id: 27_967_438,
    puid: "wikidata/27967438",
    name: "Digital Picture Exchange, version 2",
    extensions: &["dpx"],
    media_types: &["image/x-dpx"],
    internal_signatures: &[],
    related_formats: &[],
};
