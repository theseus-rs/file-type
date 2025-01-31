use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_52060199: FileFormat = FileFormat {
    id: 52_060_199,
    puid: "wikidata/52060199",
    name: "Paint Shop Pro Image, version 7",
    extensions: &["psp", "pspimage"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
