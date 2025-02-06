use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1312725: FileFormat = FileFormat {
    id: 1_312_725,
    source_type: SourceType::Wikidata,
    name: "local shared object",
    extensions: &["sol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
