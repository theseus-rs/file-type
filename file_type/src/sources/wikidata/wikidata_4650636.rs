use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4650636: FileFormat = FileFormat {
    id: 4_650_636,
    puid: "wikidata/4650636",
    name: "ACE file format",
    extensions: &["ace"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
