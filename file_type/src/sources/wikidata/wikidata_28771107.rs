use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28771107: FileFormat = FileFormat {
    id: 28_771_107,
    source_type: SourceType::Wikidata,
    name: "MAT-file, Level 4",
    extensions: &["mat"],
    media_types: &["application/x-matlab-data"],
    signatures: &[],
    related_formats: &[],
};
