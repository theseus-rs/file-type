use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110040777: FileFormat = FileFormat {
    id: 110_040_777,
    source_type: SourceType::Wikidata,
    name: "Harvard Graphics Presentation, version 4",
    extensions: &["pr4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
