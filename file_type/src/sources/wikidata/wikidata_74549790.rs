use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74549790: FileFormat = FileFormat {
    id: 74_549_790,
    puid: "wikidata/74549790",
    name: "Expert Witness compression Format SMART disk image",
    extensions: &["s01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
