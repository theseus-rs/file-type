use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7978505: FileFormat = FileFormat {
    id: 7_978_505,
    puid: "wikidata/7978505",
    name: "Web ARChive",
    extensions: &["warc", "warc", "warc"],
    media_types: &["application/warc", "application/warc", "application/warc"],
    internal_signatures: &[],
    related_formats: &[],
};
