use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113162744: FileFormat = FileFormat {
    id: 113_162_744,
    source_type: SourceType::Wikidata,
    name: "MyDeluxeInvoices & Estimates file",
    extensions: &["inv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
