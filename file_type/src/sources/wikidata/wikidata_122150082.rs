use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122150082: FileFormat = FileFormat {
    id: 122_150_082,
    source_type: SourceType::Wikidata,
    name: "TaxAct 2008 Tax Return File",
    extensions: &["ta8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
