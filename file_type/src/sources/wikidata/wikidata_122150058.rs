use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122150058: FileFormat = FileFormat {
    id: 122_150_058,
    source_type: SourceType::Wikidata,
    name: "TaxAct 2007 Tax Return File",
    extensions: &["ta7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
