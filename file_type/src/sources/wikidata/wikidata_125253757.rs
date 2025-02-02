use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125253757: FileFormat = FileFormat {
    id: 125_253_757,
    source_type: SourceType::Wikidata,
    name: "Cytoscape Exchange Format",
    extensions: &["cx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
