use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1429108: FileFormat = FileFormat {
    id: 1_429_108,
    puid: "wikidata/1429108",
    name: "NZB",
    extensions: &["nzb"],
    media_types: &["application/x-nzb"],
    internal_signatures: &[],
    related_formats: &[],
};
