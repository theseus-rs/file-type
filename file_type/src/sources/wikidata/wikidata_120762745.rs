use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120762745: FileFormat = FileFormat {
    id: 120_762_745,
    source_type: SourceType::Wikidata,
    name: "Topo USA 4.0 File",
    extensions: &["tp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
