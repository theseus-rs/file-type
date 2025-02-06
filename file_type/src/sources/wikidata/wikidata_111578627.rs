use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111578627: FileFormat = FileFormat {
    id: 111_578_627,
    source_type: SourceType::Wikidata,
    name: "Z Print Build File",
    extensions: &["zbd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
