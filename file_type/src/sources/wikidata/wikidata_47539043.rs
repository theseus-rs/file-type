use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47539043: FileFormat = FileFormat {
    id: 47_539_043,
    source_type: SourceType::Wikidata,
    name: "AutoCAD dbConnect Query Set",
    extensions: &["dbq"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
