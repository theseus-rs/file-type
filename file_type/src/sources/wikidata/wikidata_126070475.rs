use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126070475: FileFormat = FileFormat {
    id: 126_070_475,
    puid: "wikidata/126070475",
    name: "Sibelius Scorch",
    extensions: &["sco"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
