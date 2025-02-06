use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967120: FileFormat = FileFormat {
    id: 27_967_120,
    source_type: SourceType::Wikidata,
    name: "Brian Postma SoundMon v1.x module",
    extensions: &["bp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
