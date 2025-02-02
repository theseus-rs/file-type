use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600764: FileFormat = FileFormat {
    id: 28_600_764,
    source_type: SourceType::Wikidata,
    name: "Electronic Arts MOV",
    extensions: &["mov"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
