use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113544510: FileFormat = FileFormat {
    id: 113_544_510,
    puid: "wikidata/113544510",
    name: "PowerGraphics Image File",
    extensions: &["pgr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
