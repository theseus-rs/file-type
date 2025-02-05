use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207457: FileFormat = FileFormat {
    id: 28_207_457,
    source_type: SourceType::Wikidata,
    name: "Vista Plain File Format",
    extensions: &["vp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
