use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7555481: FileFormat = FileFormat {
    id: 7_555_481,
    puid: "wikidata/7555481",
    name: "sol",
    extensions: &["sol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
