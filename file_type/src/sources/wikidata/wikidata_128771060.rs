use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_128771060: FileFormat = FileFormat {
    id: 128_771_060,
    source_type: SourceType::Wikidata,
    name: "Chapel source code file",
    extensions: &["chpl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
