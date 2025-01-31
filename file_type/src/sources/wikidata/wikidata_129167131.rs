use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129167131: FileFormat = FileFormat {
    id: 129_167_131,
    puid: "wikidata/129167131",
    name: "Evoque file format",
    extensions: &["evoque"],
    media_types: &["application/x-evoque"],
    internal_signatures: &[],
    related_formats: &[],
};
