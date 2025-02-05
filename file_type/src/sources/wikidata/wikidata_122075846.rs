use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122075846: FileFormat = FileFormat {
    id: 122_075_846,
    source_type: SourceType::Wikidata,
    name: "Finale Lesson File",
    extensions: &["lsn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
