use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100666758: FileFormat = FileFormat {
    id: 100_666_758,
    source_type: SourceType::Wikidata,
    name: "Apple iWork Pages, version 9",
    extensions: &["pages"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
