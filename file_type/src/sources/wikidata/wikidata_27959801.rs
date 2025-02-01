use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959801: FileFormat = FileFormat {
    id: 27_959_801,
    puid: "wikidata/27959801",
    name: "Ableton Groove File",
    extensions: &["agr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
