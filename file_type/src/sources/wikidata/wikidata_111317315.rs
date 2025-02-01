use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317315: FileFormat = FileFormat {
    id: 111_317_315,
    puid: "wikidata/111317315",
    name: "iPhone ring-tone",
    extensions: &["m4r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
