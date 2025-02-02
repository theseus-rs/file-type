use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538961: FileFormat = FileFormat {
    id: 47_538_961,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Menu Resource File",
    extensions: &["mnr", "mnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
