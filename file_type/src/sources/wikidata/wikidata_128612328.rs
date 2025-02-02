use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128612328: FileFormat = FileFormat {
    id: 128_612_328,
    source_type: SourceType::Wikidata,
    name: "Arturo file format",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
