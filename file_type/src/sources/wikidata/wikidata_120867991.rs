use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_120867991: FileFormat = FileFormat {
    id: 120_867_991,
    source_type: SourceType::Wikidata,
    name: "Cumulus Backup File",
    extensions: &["bak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
