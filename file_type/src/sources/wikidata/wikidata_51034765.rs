use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51034765: FileFormat = FileFormat {
    id: 51_034_765,
    puid: "wikidata/51034765",
    name: "Paint Shop Pro Image, version 10",
    extensions: &["pspimage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
