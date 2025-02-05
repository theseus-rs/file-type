use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205965: FileFormat = FileFormat {
    id: 28_205_965,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Green Channel",
    extensions: &["img"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
