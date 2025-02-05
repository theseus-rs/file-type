use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975892: FileFormat = FileFormat {
    id: 28_975_892,
    source_type: SourceType::Wikidata,
    name: "Force Control File",
    extensions: &["frc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
