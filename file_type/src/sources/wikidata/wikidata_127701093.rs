use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127701093: FileFormat = FileFormat {
    id: 127_701_093,
    puid: "wikidata/127701093",
    name: "Hack source code file",
    extensions: &["hack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
