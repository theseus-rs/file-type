use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856273: FileFormat = FileFormat {
    id: 105_856_273,
    puid: "wikidata/105856273",
    name: "Virtuoso Display Resource File (with rem)",
    extensions: &["drf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
