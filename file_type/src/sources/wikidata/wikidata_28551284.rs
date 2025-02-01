use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551284: FileFormat = FileFormat {
    id: 28_551_284,
    puid: "wikidata/28551284",
    name: "Adobe CMYK Setup File",
    extensions: &["api"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
