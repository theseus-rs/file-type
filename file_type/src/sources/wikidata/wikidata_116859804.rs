use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116859804: FileFormat = FileFormat {
    id: 116_859_804,
    puid: "wikidata/116859804",
    name: "Peachtree Vendor List",
    extensions: &["csv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
