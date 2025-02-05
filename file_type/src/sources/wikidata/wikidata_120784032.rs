use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120784032: FileFormat = FileFormat {
    id: 120_784_032,
    source_type: SourceType::Wikidata,
    name: "3-D TopoQuads 2.0 File",
    extensions: &["tq2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
