use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110984245: FileFormat = FileFormat {
    id: 110_984_245,
    source_type: SourceType::Wikidata,
    name: "Video Paint File",
    extensions: &["uvp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
