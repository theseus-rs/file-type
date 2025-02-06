use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263049: FileFormat = FileFormat {
    id: 111_263_049,
    source_type: SourceType::Wikidata,
    name: "ActiveMovie streaming format",
    extensions: &["asf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
