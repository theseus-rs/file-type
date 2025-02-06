use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112819749: FileFormat = FileFormat {
    id: 112_819_749,
    source_type: SourceType::Wikidata,
    name: "Detailer 3D File",
    extensions: &["vdu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
