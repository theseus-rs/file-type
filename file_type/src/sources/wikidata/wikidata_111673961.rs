use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111673961: FileFormat = FileFormat {
    id: 111_673_961,
    puid: "wikidata/111673961",
    name: "Kingsoft Writer Template",
    extensions: &["wpt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
