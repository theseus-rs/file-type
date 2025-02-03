use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_26385770: FileFormat = FileFormat {
    id: 26_385_770,
    source_type: SourceType::Wikidata,
    name: "Still Picture Interchange File Format",
    extensions: &["jpeg", "jpg", "spf", "spiff"],
    media_types: &["image/jpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
