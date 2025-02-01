use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771300: FileFormat = FileFormat {
    id: 28_771_300,
    puid: "wikidata/28771300",
    name: "Markdeep",
    extensions: &["md.html"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
