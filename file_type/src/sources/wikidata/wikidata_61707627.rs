use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61707627: FileFormat = FileFormat {
    id: 61_707_627,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Database File Locking Information",
    extensions: &["dwl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
