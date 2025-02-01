use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3564764: FileFormat = FileFormat {
    id: 3_564_764,
    puid: "wikidata/3564764",
    name: "Word Perfect Graphic 1.0",
    extensions: &["wpg"],
    media_types: &["application/x-wpg"],
    internal_signatures: &[],
    related_formats: &[],
};
