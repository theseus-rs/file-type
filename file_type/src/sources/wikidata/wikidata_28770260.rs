use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28770260: FileFormat = FileFormat {
    id: 28_770_260,
    puid: "wikidata/28770260",
    name: "Hugo",
    extensions: &["hex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
