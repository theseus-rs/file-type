use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87987058: FileFormat = FileFormat {
    id: 87_987_058,
    source_type: SourceType::Wikidata,
    name: "CorelCHART Document 5",
    extensions: &["cch"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
