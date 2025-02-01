use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119846012: FileFormat = FileFormat {
    id: 119_846_012,
    puid: "wikidata/119846012",
    name: "Quicken Data File (Macintosh)",
    extensions: &["qdfm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
