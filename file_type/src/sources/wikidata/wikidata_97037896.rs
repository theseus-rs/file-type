use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_97037896: FileFormat = FileFormat {
    id: 97_037_896,
    source_type: SourceType::Wikidata,
    name: "Personal Icon",
    extensions: &["picon"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
