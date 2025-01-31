use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61707627: FileFormat = FileFormat {
    id: 61_707_627,
    puid: "wikidata/61707627",
    name: "AutoCAD Database File Locking Information",
    extensions: &["dwl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
