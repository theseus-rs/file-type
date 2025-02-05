use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118146490: FileFormat = FileFormat {
    id: 118_146_490,
    source_type: SourceType::Wikidata,
    name: "Edge-couple Symmetric Stripline File",
    extensions: &["tl5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
