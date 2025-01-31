use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126811644: FileFormat = FileFormat {
    id: 126_811_644,
    puid: "wikidata/126811644",
    name: "Fenix Graphics Collection File",
    extensions: &["fpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
