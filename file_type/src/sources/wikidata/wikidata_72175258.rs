use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72175258: FileFormat = FileFormat {
    id: 72_175_258,
    source_type: SourceType::Wikidata,
    name: "Kaspersky Anti-Virus signature bases",
    extensions: &["kdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
