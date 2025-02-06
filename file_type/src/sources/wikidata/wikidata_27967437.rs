use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967437: FileFormat = FileFormat {
    id: 27_967_437,
    source_type: SourceType::Wikidata,
    name: "Digital Picture Exchange, version 1",
    extensions: &["dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
