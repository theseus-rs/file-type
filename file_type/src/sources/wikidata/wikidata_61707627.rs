use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61707627: FileFormat = FileFormat {
    id: 61_707_627,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Database File Locking Information",
    extensions: &["dwl"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
