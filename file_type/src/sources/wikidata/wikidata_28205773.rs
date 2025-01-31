use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205773: FileFormat = FileFormat {
    id: 28_205_773,
    puid: "wikidata/28205773",
    name: "BioRad confocal image",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
