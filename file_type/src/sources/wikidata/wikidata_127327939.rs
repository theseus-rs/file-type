use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127327939: FileFormat = FileFormat {
    id: 127_327_939,
    source_type: SourceType::Wikidata,
    name: "COBOL Source Code File",
    extensions: &["cbl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
