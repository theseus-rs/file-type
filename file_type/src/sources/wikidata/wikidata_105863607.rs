use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863607: FileFormat = FileFormat {
    id: 105_863_607,
    source_type: SourceType::Wikidata,
    name: "Metalink (v4)",
    extensions: &["meta4"],
    media_types: &["application/metalink4+xml"],
    signatures: &[],
    related_formats: &[],
};
