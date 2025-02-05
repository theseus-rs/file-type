use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125253619: FileFormat = FileFormat {
    id: 125_253_619,
    source_type: SourceType::Wikidata,
    name: "Simple interaction file",
    extensions: &["sif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
