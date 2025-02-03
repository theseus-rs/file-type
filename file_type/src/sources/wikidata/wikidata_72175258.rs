use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_72175258: FileFormat = FileFormat {
    id: 72_175_258,
    source_type: SourceType::Wikidata,
    name: "Kaspersky Anti-Virus signature bases",
    extensions: &["kdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
