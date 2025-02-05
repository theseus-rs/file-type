use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113402233: FileFormat = FileFormat {
    id: 113_402_233,
    source_type: SourceType::Wikidata,
    name: "ZBrush MatCap",
    extensions: &["zmt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
