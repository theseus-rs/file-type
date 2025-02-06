use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445596: FileFormat = FileFormat {
    id: 28_445_596,
    source_type: SourceType::Wikidata,
    name: "APD",
    extensions: &["apd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
