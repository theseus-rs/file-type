use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87402788: FileFormat = FileFormat {
    id: 87_402_788,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Temporary File",
    extensions: &["ac$"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
