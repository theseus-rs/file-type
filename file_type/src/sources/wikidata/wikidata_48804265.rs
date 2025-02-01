use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48804265: FileFormat = FileFormat {
    id: 48_804_265,
    puid: "wikidata/48804265",
    name: "Paradox Database Memo Field",
    extensions: &["dbq", "mb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
