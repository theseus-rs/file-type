use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125916061: FileFormat = FileFormat {
    id: 125_916_061,
    source_type: SourceType::Wikidata,
    name: "SolidWorks Material Database File",
    extensions: &["sldmat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
