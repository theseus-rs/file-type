use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96054590: FileFormat = FileFormat {
    id: 96_054_590,
    puid: "wikidata/96054590",
    name: "Macromolecular Crystallographic Information File",
    extensions: &["mmcif"],
    media_types: &["chemical/x-mmcif"],
    internal_signatures: &[],
    related_formats: &[],
};
