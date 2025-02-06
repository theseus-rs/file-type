use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113376732: FileFormat = FileFormat {
    id: 113_376_732,
    source_type: SourceType::Wikidata,
    name: "Easy CD Creator Layout, version 5-6",
    extensions: &["cl5", "rcl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
