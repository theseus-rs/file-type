use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104897515: FileFormat = FileFormat {
    id: 104_897_515,
    source_type: SourceType::Wikidata,
    name: "Propellerhead Reason ReFill Sound Bank",
    extensions: &["rfl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
