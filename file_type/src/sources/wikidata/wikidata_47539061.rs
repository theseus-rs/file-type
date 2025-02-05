use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47539061: FileFormat = FileFormat {
    id: 47_539_061,
    source_type: SourceType::Wikidata,
    name: "AutoCAD dbConnect Template Set",
    extensions: &["dbt"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
