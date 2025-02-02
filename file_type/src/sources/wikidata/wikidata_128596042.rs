use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128596042: FileFormat = FileFormat {
    id: 128_596_042,
    source_type: SourceType::Wikidata,
    name: "Aheui file format",
    extensions: &["aheui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
