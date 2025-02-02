use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650337: FileFormat = FileFormat {
    id: 29_650_337,
    source_type: SourceType::Wikidata,
    name: "PUPA Font Format, version 2",
    extensions: &["pf2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
