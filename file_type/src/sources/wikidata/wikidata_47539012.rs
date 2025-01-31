use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539012: FileFormat = FileFormat {
    id: 47_539_012,
    puid: "wikidata/47539012",
    name: "AutoCAD Drawing Template",
    extensions: &["dwt"],
    media_types: &["application/x-autocad"],
    internal_signatures: &[],
    related_formats: &[],
};
