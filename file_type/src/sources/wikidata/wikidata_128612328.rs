use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128612328: FileFormat = FileFormat {
    id: 128_612_328,
    source_type: SourceType::Wikidata,
    name: "Arturo file format",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
