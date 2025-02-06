use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27824015: FileFormat = FileFormat {
    id: 27_824_015,
    source_type: SourceType::Wikidata,
    name: "ar, BSD variant",
    extensions: &["a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
