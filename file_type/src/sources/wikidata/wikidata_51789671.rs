use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51789671: FileFormat = FileFormat {
    id: 51_789_671,
    source_type: SourceType::Wikidata,
    name: "AutoCAD External Database Configuration File",
    extensions: &["udl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
