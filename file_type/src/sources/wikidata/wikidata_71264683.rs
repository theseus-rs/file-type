use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71264683: FileFormat = FileFormat {
    id: 71_264_683,
    puid: "wikidata/71264683",
    name: "Hippel module",
    extensions: &["hip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
