use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111262641: FileFormat = FileFormat {
    id: 111_262_641,
    puid: "wikidata/111262641",
    name: "Muon DS404 bank file",
    extensions: &["404"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
