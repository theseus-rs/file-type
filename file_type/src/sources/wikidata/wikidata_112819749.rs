use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112819749: FileFormat = FileFormat {
    id: 112_819_749,
    source_type: SourceType::Wikidata,
    name: "Detailer 3D File",
    extensions: &["vdu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
