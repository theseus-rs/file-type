use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538961: FileFormat = FileFormat {
    id: 47_538_961,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Menu Resource File",
    extensions: &["mnr", "mnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
