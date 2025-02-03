use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120784032: FileFormat = FileFormat {
    id: 120_784_032,
    source_type: SourceType::Wikidata,
    name: "3-D TopoQuads 2.0 File",
    extensions: &["tq2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
