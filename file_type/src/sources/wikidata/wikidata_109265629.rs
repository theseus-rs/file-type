use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109265629: FileFormat = FileFormat {
    id: 109_265_629,
    puid: "wikidata/109265629",
    name: "MultiMate Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
