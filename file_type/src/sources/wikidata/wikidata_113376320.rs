use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113376320: FileFormat = FileFormat {
    id: 113_376_320,
    puid: "wikidata/113376320",
    name: "XL-Paint format",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
