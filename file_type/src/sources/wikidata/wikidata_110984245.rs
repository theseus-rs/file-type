use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110984245: FileFormat = FileFormat {
    id: 110_984_245,
    puid: "wikidata/110984245",
    name: "Video Paint File",
    extensions: &["uvp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
