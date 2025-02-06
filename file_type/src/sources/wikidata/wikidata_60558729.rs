use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60558729: FileFormat = FileFormat {
    id: 60_558_729,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Painting, version 2",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
