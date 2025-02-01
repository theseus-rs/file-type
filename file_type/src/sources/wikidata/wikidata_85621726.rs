use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_85621726: FileFormat = FileFormat {
    id: 85_621_726,
    puid: "wikidata/85621726",
    name: "PFS:First Choice Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
