use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120762588: FileFormat = FileFormat {
    id: 120_762_588,
    puid: "wikidata/120762588",
    name: "Topo USA File",
    extensions: &["tpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
