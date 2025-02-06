use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27526426: FileFormat = FileFormat {
    id: 27_526_426,
    source_type: SourceType::Wikidata,
    name: "Broadcast Wave Format, version 0",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
