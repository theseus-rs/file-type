use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967128: FileFormat = FileFormat {
    id: 27_967_128,
    source_type: SourceType::Wikidata,
    name: "DMC",
    extensions: &["dmc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
