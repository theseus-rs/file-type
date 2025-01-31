use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122974666: FileFormat = FileFormat {
    id: 122_974_666,
    puid: "wikidata/122974666",
    name: "CAMP",
    extensions: &["camp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
