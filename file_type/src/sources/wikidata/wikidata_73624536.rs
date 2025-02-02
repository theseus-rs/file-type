use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_73624536: FileFormat = FileFormat {
    id: 73_624_536,
    source_type: SourceType::Wikidata,
    name: "Intuit QuickBooks Backup",
    extensions: &["qbb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
