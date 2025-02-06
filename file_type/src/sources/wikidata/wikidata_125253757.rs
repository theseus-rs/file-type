use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125253757: FileFormat = FileFormat {
    id: 125_253_757,
    source_type: SourceType::Wikidata,
    name: "Cytoscape Exchange Format",
    extensions: &["cx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
