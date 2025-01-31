use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539001: FileFormat = FileFormat {
    id: 47_539_001,
    puid: "wikidata/47539001",
    name: "AutoCAD Linetype Definition File",
    extensions: &["lin"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};
