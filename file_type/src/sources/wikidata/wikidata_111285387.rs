use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111285387: FileFormat = FileFormat {
    id: 111_285_387,
    source_type: SourceType::Wikidata,
    name: "Sound Tools HCOM format",
    extensions: &["hcom"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
