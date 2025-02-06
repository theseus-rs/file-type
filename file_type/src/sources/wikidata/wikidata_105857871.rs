use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857871: FileFormat = FileFormat {
    id: 105_857_871,
    source_type: SourceType::Wikidata,
    name: "ILINK linker Configuration",
    extensions: &["icf"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
