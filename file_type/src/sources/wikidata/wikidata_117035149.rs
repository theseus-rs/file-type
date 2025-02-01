use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117035149: FileFormat = FileFormat {
    id: 117_035_149,
    puid: "wikidata/117035149",
    name: "TurboCAD for Windows ASCII File",
    extensions: &["tcx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
