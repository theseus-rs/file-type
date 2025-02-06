use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121758732: FileFormat = FileFormat {
    id: 121_758_732,
    source_type: SourceType::Wikidata,
    name: "Family Tree Maker FTMB Backup File",
    extensions: &["ftmb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
