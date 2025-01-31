use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125208012: FileFormat = FileFormat {
    id: 125_208_012,
    puid: "wikidata/125208012",
    name: "TaskJuggler project",
    extensions: &["tjp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
