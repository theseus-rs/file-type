use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87987058: FileFormat = FileFormat {
    id: 87_987_058,
    source_type: SourceType::Wikidata,
    name: "CorelCHART Document 5",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
