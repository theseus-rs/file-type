use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_84943071: FileFormat = FileFormat {
    id: 84_943_071,
    source_type: SourceType::Wikidata,
    name: "Sony PictureGear Studio Binder",
    extensions: &["bxt", "bxu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
