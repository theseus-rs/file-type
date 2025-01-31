use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119496138: FileFormat = FileFormat {
    id: 119_496_138,
    puid: "wikidata/119496138",
    name: "WinFax format",
    extensions: &["wfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
