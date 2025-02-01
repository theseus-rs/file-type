use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50498413: FileFormat = FileFormat {
    id: 50_498_413,
    puid: "wikidata/50498413",
    name: "Draco File Format",
    extensions: &["drc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
