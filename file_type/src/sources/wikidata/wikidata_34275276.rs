use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34275276: FileFormat = FileFormat {
    id: 34_275_276,
    source_type: SourceType::Wikidata,
    name: "Numbers Zipped",
    extensions: &["numbers.zip"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
