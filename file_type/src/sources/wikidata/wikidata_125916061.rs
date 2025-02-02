use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125916061: FileFormat = FileFormat {
    id: 125_916_061,
    source_type: SourceType::Wikidata,
    name: "SolidWorks Material Database File",
    extensions: &["sldmat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
