use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52830687: FileFormat = FileFormat {
    id: 52_830_687,
    puid: "wikidata/52830687",
    name: "Paint Shop Pro Image, version 8",
    extensions: &["pspimage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
