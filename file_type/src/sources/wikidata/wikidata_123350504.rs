use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123350504: FileFormat = FileFormat {
    id: 123_350_504,
    source_type: SourceType::Wikidata,
    name: "RootsMagic Chart file",
    extensions: &["rmc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
