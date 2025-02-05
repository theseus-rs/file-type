use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29650337: FileFormat = FileFormat {
    id: 29_650_337,
    source_type: SourceType::Wikidata,
    name: "PUPA Font Format, version 2",
    extensions: &["pf2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
