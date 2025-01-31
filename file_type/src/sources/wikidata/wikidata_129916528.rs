use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129916528: FileFormat = FileFormat {
    id: 129_916_528,
    puid: "wikidata/129916528",
    name: "Janet file format",
    extensions: &["janet", "janet"],
    media_types: &["application/x-janet", "text/x-janet"],
    internal_signatures: &[],
    related_formats: &[],
};
