use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27861323: FileFormat = FileFormat {
    id: 27_861_323,
    source_type: SourceType::Wikidata,
    name: "Windows Prefetch File, version 23",
    extensions: &["pf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
