use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123350504: FileFormat = FileFormat {
    id: 123_350_504,
    source_type: SourceType::Wikidata,
    name: "RootsMagic Chart file",
    extensions: &["rmc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
