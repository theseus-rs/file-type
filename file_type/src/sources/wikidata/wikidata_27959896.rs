use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959896: FileFormat = FileFormat {
    id: 27_959_896,
    source_type: SourceType::Wikidata,
    name: "Nuendo arrangement",
    extensions: &["npr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
