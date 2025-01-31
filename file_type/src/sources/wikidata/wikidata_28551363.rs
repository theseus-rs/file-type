use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551363: FileFormat = FileFormat {
    id: 28_551_363,
    puid: "wikidata/28551363",
    name: "Adobe Levels File",
    extensions: &["alv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
