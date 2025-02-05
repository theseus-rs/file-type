use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64859108: FileFormat = FileFormat {
    id: 64_859_108,
    source_type: SourceType::Wikidata,
    name: "Family Tree Maker Backup file format",
    extensions: &["fbk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
