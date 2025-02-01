use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_92440742: FileFormat = FileFormat {
    id: 92_440_742,
    puid: "wikidata/92440742",
    name: "Spider 2D image",
    extensions: &["spider"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
