use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120867991: FileFormat = FileFormat {
    id: 120_867_991,
    source_type: SourceType::Wikidata,
    name: "Cumulus Backup File",
    extensions: &["bak"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
