use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28975863: FileFormat = FileFormat {
    id: 28_975_863,
    source_type: SourceType::Wikidata,
    name: "OOGL Bezier Surface BEZ",
    extensions: &["bez"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
