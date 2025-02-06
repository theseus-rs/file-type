use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445581: FileFormat = FileFormat {
    id: 28_445_581,
    source_type: SourceType::Wikidata,
    name: "ADRIFT",
    extensions: &["taf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
