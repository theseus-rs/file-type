use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_26385770: FileFormat = FileFormat {
    id: 26_385_770,
    source_type: SourceType::Wikidata,
    name: "Still Picture Interchange File Format",
    extensions: &["jpeg", "jpg", "spf", "spiff"],
    media_types: &["image/jpeg"],
    signatures: &[],
    related_formats: &[],
};
