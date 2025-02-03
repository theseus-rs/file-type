use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29000735: FileFormat = FileFormat {
    id: 29_000_735,
    source_type: SourceType::Wikidata,
    name: "VOL",
    extensions: &["vol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
