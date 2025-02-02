use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120042266: FileFormat = FileFormat {
    id: 120_042_266,
    source_type: SourceType::Wikidata,
    name: "Cheyenne Backup Script",
    extensions: &["asx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
