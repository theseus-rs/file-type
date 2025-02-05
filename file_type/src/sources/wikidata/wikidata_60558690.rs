use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60558690: FileFormat = FileFormat {
    id: 60_558_690,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Database, version 2",
    extensions: &["cwk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
