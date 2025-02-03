use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863736: FileFormat = FileFormat {
    id: 105_863_736,
    source_type: SourceType::Wikidata,
    name: "MuSiCa text music format (with rem)",
    extensions: &["msd"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
