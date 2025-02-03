use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127327939: FileFormat = FileFormat {
    id: 127_327_939,
    source_type: SourceType::Wikidata,
    name: "COBOL Source Code File",
    extensions: &["cbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
