use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87402788: FileFormat = FileFormat {
    id: 87_402_788,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Temporary File",
    extensions: &["ac$"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
