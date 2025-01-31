use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28447338: FileFormat = FileFormat {
    id: 28_447_338,
    puid: "wikidata/28447338",
    name: "Digital Document",
    extensions: &["ddoc"],
    media_types: &["application/ddoc"],
    internal_signatures: &[],
    related_formats: &[],
};
