use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128034125: FileFormat = FileFormat {
    id: 128_034_125,
    source_type: SourceType::Wikidata,
    name: "Rexx source code file",
    extensions: &["arexx", "rex", "rexx", "rx"],
    media_types: &["text/x-rexx"],
    signatures: &[],
    related_formats: &[],
};
