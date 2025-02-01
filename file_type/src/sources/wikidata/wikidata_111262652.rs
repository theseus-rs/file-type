use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262652: FileFormat = FileFormat {
    id: 111_262_652,
    puid: "wikidata/111262652",
    name: "Muon DS404 patch file",
    extensions: &["404"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
