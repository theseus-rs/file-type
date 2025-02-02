use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857871: FileFormat = FileFormat {
    id: 105_857_871,
    source_type: SourceType::Wikidata,
    name: "ILINK linker Configuration",
    extensions: &["icf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
