use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344013: FileFormat = FileFormat {
    id: 28_344_013,
    source_type: SourceType::Wikidata,
    name: "BACKUP",
    extensions: &["@@@"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
