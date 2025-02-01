use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27350185: FileFormat = FileFormat {
    id: 27_350_185,
    puid: "wikidata/27350185",
    name: "ADRG Test Patch Image File",
    extensions: &["cph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
