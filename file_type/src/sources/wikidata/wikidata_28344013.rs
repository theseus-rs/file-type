use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344013: FileFormat = FileFormat {
    id: 28_344_013,
    source_type: SourceType::Wikidata,
    name: "BACKUP",
    extensions: &["@@@"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
