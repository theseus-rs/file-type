use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100596765: FileFormat = FileFormat {
    id: 100_596_765,
    source_type: SourceType::Wikidata,
    name: "Minitab Project, version 12-13",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
