use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122150098: FileFormat = FileFormat {
    id: 122_150_098,
    source_type: SourceType::Wikidata,
    name: "TaxAct 2008 Tax Return Backup File",
    extensions: &["ba8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
