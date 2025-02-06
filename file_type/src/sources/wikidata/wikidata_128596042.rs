use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128596042: FileFormat = FileFormat {
    id: 128_596_042,
    source_type: SourceType::Wikidata,
    name: "Aheui file format",
    extensions: &["aheui"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
