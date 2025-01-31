use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66004695: FileFormat = FileFormat {
    id: 66_004_695,
    puid: "wikidata/66004695",
    name: "ImageStyler File",
    extensions: &["ist"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
