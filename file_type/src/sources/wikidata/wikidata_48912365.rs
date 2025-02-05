use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48912365: FileFormat = FileFormat {
    id: 48_912_365,
    source_type: SourceType::Wikidata,
    name: "InterBase Database",
    extensions: &["gdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
