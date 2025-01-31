use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67206676: FileFormat = FileFormat {
    id: 67_206_676,
    puid: "wikidata/67206676",
    name: "TurboCAD for Windows Drawing file",
    extensions: &["tcw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
