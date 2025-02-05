use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_56315514: FileFormat = FileFormat {
    id: 56_315_514,
    source_type: SourceType::Wikidata,
    name: "UML diagram",
    extensions: &["uml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
