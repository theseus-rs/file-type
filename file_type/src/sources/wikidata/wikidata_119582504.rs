use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119582504: FileFormat = FileFormat {
    id: 119_582_504,
    puid: "wikidata/119582504",
    name: "EMLX",
    extensions: &["emlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
