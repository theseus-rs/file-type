use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27526504: FileFormat = FileFormat {
    id: 27_526_504,
    source_type: SourceType::Wikidata,
    name: "Broadcast Wave Format, version 2",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
