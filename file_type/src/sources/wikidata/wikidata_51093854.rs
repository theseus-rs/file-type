use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51093854: FileFormat = FileFormat {
    id: 51_093_854,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Plot Configuration File, version 2000",
    extensions: &["pc3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
