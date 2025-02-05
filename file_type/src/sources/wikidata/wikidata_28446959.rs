use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28446959: FileFormat = FileFormat {
    id: 28_446_959,
    source_type: SourceType::Wikidata,
    name: "Binary Document",
    extensions: &["bdoc"],
    media_types: &["application/vnd.bdoc-1.0"],
    signatures: &[],
    related_formats: &[],
};
