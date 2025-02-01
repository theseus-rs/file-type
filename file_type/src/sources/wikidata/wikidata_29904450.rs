use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904450: FileFormat = FileFormat {
    id: 29_904_450,
    puid: "wikidata/29904450",
    name: "Presentation Manager Metafile",
    extensions: &["met"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
