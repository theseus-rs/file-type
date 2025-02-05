use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131395745: FileFormat = FileFormat {
    id: 131_395_745,
    source_type: SourceType::Wikidata,
    name: "VGL source code file",
    extensions: &["rpf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
