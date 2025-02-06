use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73624536: FileFormat = FileFormat {
    id: 73_624_536,
    source_type: SourceType::Wikidata,
    name: "Intuit QuickBooks Backup",
    extensions: &["qbb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
