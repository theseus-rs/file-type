use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539005: FileFormat = FileFormat {
    id: 47_539_005,
    puid: "wikidata/47539005",
    name: "AutoCAD Font Mapping Table",
    extensions: &["fmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
