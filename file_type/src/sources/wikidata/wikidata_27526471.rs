use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27526471: FileFormat = FileFormat {
    id: 27_526_471,
    source_type: SourceType::Wikidata,
    name: "Broadcast Wave Format, version 1",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
