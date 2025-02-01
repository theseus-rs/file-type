use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551355: FileFormat = FileFormat {
    id: 28_551_355,
    puid: "wikidata/28551355",
    name: "Adobe Hue/Saturation File",
    extensions: &["ahu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
