use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122408089: FileFormat = FileFormat {
    id: 122_408_089,
    source_type: SourceType::Wikidata,
    name: "PlayStation Debug Executable",
    extensions: &["pse"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
