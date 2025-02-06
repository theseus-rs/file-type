use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111263191: FileFormat = FileFormat {
    id: 111_263_191,
    source_type: SourceType::Wikidata,
    name: "Audio CD compatible raw data",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
