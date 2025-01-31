use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61974843: FileFormat = FileFormat {
    id: 61_974_843,
    puid: "wikidata/61974843",
    name: "FoxPro Compound Index File",
    extensions: &["cdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
