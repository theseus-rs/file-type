use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1258721: FileFormat = FileFormat {
    id: 1_258_721,
    puid: "wikidata/1258721",
    name: "NFO",
    extensions: &["nfo"],
    media_types: &["text/x-nfo"],
    internal_signatures: &[],
    related_formats: &[],
};
