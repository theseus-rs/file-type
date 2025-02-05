use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967183: FileFormat = FileFormat {
    id: 27_967_183,
    source_type: SourceType::Wikidata,
    name: "FastTracker module",
    extensions: &["ft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
