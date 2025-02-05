use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113376526: FileFormat = FileFormat {
    id: 113_376_526,
    source_type: SourceType::Wikidata,
    name: "RED Thumbnail File",
    extensions: &["rtn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
