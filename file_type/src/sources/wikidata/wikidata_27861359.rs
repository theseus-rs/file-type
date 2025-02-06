use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861359: FileFormat = FileFormat {
    id: 27_861_359,
    source_type: SourceType::Wikidata,
    name: "Windows Prefetch File, version 30",
    extensions: &["pf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
