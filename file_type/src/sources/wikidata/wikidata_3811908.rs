use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_3811908: FileFormat = FileFormat {
    id: 3_811_908,
    source_type: SourceType::Wikidata,
    name: "karaoke file",
    extensions: &["kar"],
    media_types: &["audio/midi"],
    signatures: &[],
    related_formats: &[],
};
