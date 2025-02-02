use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_60558690: FileFormat = FileFormat {
    id: 60_558_690,
    source_type: SourceType::Wikidata,
    name: "ClarisWorks Database, version 2",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
