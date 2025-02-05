use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122150082: FileFormat = FileFormat {
    id: 122_150_082,
    source_type: SourceType::Wikidata,
    name: "TaxAct 2008 Tax Return File",
    extensions: &["ta8"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
