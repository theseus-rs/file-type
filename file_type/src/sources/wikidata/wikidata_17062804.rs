use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17062804: FileFormat = FileFormat {
    id: 17_062_804,
    puid: "wikidata/17062804",
    name: "Klip",
    extensions: &["klip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
