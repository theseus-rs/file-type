use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111496391: FileFormat = FileFormat {
    id: 111_496_391,
    puid: "wikidata/111496391",
    name: "Visual Basic Project Workspace File",
    extensions: &["vbw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
