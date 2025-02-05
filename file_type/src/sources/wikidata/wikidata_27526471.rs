use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27526471: FileFormat = FileFormat {
    id: 27_526_471,
    source_type: SourceType::Wikidata,
    name: "Broadcast Wave Format, version 1",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
