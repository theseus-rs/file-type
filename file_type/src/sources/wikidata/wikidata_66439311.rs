use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66439311: FileFormat = FileFormat {
    id: 66_439_311,
    source_type: SourceType::Wikidata,
    name: "Navy DIF",
    extensions: &["dif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
