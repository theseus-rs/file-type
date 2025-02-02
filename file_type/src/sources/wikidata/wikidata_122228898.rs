use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122228898: FileFormat = FileFormat {
    id: 122_228_898,
    source_type: SourceType::Wikidata,
    name: "Oracle Password Hash",
    extensions: &["orc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
