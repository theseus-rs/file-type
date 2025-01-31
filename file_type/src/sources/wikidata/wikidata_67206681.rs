use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67206681: FileFormat = FileFormat {
    id: 67_206_681,
    puid: "wikidata/67206681",
    name: "TurboCAD Template",
    extensions: &["tct"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
