use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6059108: FileFormat = FileFormat {
    id: 6_059_108,
    source_type: SourceType::Wikidata,
    name: "Intuit Interchange Format",
    extensions: &["iif"],
    media_types: &["application/qbooks", "application/qbookspro", "text/iif"],
    signatures: &[],
    related_formats: &[],
};
