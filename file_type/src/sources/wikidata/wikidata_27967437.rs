use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967437: FileFormat = FileFormat {
    id: 27_967_437,
    puid: "wikidata/27967437",
    name: "Digital Picture Exchange, version 1",
    extensions: &["dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
