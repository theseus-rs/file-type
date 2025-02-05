use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125415086: FileFormat = FileFormat {
    id: 125_415_086,
    source_type: SourceType::Wikidata,
    name: "TCM document",
    extensions: &["tcm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
