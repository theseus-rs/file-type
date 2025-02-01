use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52834540: FileFormat = FileFormat {
    id: 52_834_540,
    puid: "wikidata/52834540",
    name: "Paint Shop Pro Image, version 4",
    extensions: &["psp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
