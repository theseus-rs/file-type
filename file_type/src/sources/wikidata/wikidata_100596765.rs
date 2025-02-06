use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100596765: FileFormat = FileFormat {
    id: 100_596_765,
    source_type: SourceType::Wikidata,
    name: "Minitab Project, version 12-13",
    extensions: &["mpj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
