use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959836: FileFormat = FileFormat {
    id: 27_959_836,
    source_type: SourceType::Wikidata,
    name: "Raw FL Studio Project",
    extensions: &["flp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
