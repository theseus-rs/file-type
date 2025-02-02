use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113162744: FileFormat = FileFormat {
    id: 113_162_744,
    source_type: SourceType::Wikidata,
    name: "MyDeluxeInvoices & Estimates file",
    extensions: &["inv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
