use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60558525: FileFormat = FileFormat {
    id: 60_558_525,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Drawing file format, version 2",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
