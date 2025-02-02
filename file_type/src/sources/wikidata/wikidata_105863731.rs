use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863731: FileFormat = FileFormat {
    id: 105_863_731,
    source_type: SourceType::Wikidata,
    name: "Mapping Specification Language (ASCII)",
    extensions: &["msl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
