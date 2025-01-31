use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122410584: FileFormat = FileFormat {
    id: 122_410_584,
    puid: "wikidata/122410584",
    name: "PowerPC Symbol File",
    extensions: &["xsym"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
