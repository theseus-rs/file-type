use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85621806: FileFormat = FileFormat {
    id: 85_621_806,
    puid: "wikidata/85621806",
    name: "PFS:First Choice Document 3",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
