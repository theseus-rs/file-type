use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109596469: FileFormat = FileFormat {
    id: 109_596_469,
    source_type: SourceType::Wikidata,
    name: "DrawPlus Template",
    extensions: &["dpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
