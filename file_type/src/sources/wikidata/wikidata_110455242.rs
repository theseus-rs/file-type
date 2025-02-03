use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110455242: FileFormat = FileFormat {
    id: 110_455_242,
    source_type: SourceType::Wikidata,
    name: "Septentrio Binary Format",
    extensions: &["sbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
