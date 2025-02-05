use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110455242: FileFormat = FileFormat {
    id: 110_455_242,
    source_type: SourceType::Wikidata,
    name: "Septentrio Binary Format",
    extensions: &["sbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
