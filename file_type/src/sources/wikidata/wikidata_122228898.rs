use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122228898: FileFormat = FileFormat {
    id: 122_228_898,
    source_type: SourceType::Wikidata,
    name: "Oracle Password Hash",
    extensions: &["orc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
