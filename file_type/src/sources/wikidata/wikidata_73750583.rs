use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_73750583: FileFormat = FileFormat {
    id: 73_750_583,
    source_type: SourceType::Wikidata,
    name: "Intuit QuickBooks for Windows",
    extensions: &["qbw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
