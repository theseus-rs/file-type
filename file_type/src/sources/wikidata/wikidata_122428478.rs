use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122428478: FileFormat = FileFormat {
    id: 122_428_478,
    source_type: SourceType::Wikidata,
    name: "Wild Photo Effects file",
    extensions: &["moo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
