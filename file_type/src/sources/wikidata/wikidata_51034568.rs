use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51034568: FileFormat = FileFormat {
    id: 51_034_568,
    puid: "wikidata/51034568",
    name: "Paint Shop Pro Image, version 9",
    extensions: &["pspimage"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
