use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_84943071: FileFormat = FileFormat {
    id: 84_943_071,
    source_type: SourceType::Wikidata,
    name: "Sony PictureGear Studio Binder",
    extensions: &["bxt", "bxu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
