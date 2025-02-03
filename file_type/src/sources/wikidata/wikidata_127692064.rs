use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127692064: FileFormat = FileFormat {
    id: 127_692_064,
    source_type: SourceType::Wikidata,
    name: "freefem format",
    extensions: &["msh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
