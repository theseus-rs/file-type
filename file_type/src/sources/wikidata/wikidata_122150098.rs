use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122150098: FileFormat = FileFormat {
    id: 122_150_098,
    source_type: SourceType::Wikidata,
    name: "TaxAct 2008 Tax Return Backup File",
    extensions: &["ba8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
