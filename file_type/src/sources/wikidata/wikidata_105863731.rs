use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863731: FileFormat = FileFormat {
    id: 105_863_731,
    source_type: SourceType::Wikidata,
    name: "Mapping Specification Language (ASCII)",
    extensions: &["msl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
