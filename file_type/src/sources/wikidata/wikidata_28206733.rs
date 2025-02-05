use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206733: FileFormat = FileFormat {
    id: 28_206_733,
    source_type: SourceType::Wikidata,
    name: "Newsroom",
    extensions: &["nsr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
