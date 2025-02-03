use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122150058: FileFormat = FileFormat {
    id: 122_150_058,
    source_type: SourceType::Wikidata,
    name: "TaxAct 2007 Tax Return File",
    extensions: &["ta7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
