use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113376320: FileFormat = FileFormat {
    id: 113_376_320,
    source_type: SourceType::Wikidata,
    name: "XL-Paint format",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
