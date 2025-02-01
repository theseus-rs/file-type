use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_84943071: FileFormat = FileFormat {
    id: 84_943_071,
    puid: "wikidata/84943071",
    name: "Sony PictureGear Studio Binder",
    extensions: &["bxt", "bxu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
