use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110040777: FileFormat = FileFormat {
    id: 110_040_777,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Presentation, version 4",
    extensions: &["pr4"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
