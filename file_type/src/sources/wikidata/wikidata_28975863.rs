use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975863: FileFormat = FileFormat {
    id: 28_975_863,
    source_type: SourceType::Wikidata,
    name: "OOGL Bezier Surface BEZ",
    extensions: &["bez"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
