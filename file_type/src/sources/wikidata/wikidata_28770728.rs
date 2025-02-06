use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28770728: FileFormat = FileFormat {
    id: 28_770_728,
    source_type: SourceType::Wikidata,
    name: "MAT-file, Level 5, version 6",
    extensions: &["mat"],
    media_types: &["application/x-matlab-data"],
    signatures: &[],
    related_formats: &[],
};
