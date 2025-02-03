use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125347324: FileFormat = FileFormat {
    id: 125_347_324,
    source_type: SourceType::Wikidata,
    name: "Binary Grid File",
    extensions: &["grb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
