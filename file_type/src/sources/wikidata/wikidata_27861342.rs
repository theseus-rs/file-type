use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27861342: FileFormat = FileFormat {
    id: 27_861_342,
    source_type: SourceType::Wikidata,
    name: "Windows Prefetch File, version 26",
    extensions: &["pf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
