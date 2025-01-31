use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47539144: FileFormat = FileFormat {
    id: 47_539_144,
    puid: "wikidata/47539144",
    name: "AutoCAD Batch Plot File, version 2000-2005",
    extensions: &["bp3"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
