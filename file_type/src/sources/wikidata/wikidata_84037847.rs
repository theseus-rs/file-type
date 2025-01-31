use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84037847: FileFormat = FileFormat {
    id: 84_037_847,
    puid: "wikidata/84037847",
    name: "WARC 1.1",
    extensions: &["warc"],
    media_types: &["application/warc"],
    internal_signatures: &[],
    related_formats: &[],
};
