use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51718111: FileFormat = FileFormat {
    id: 51_718_111,
    source_type: SourceType::Wikidata,
    name: "AutoCAD ACIS Export File",
    extensions: &["sat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
