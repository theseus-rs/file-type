use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863736: FileFormat = FileFormat {
    id: 105_863_736,
    source_type: SourceType::Wikidata,
    name: "MuSiCa text music format (with rem)",
    extensions: &["msd"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
