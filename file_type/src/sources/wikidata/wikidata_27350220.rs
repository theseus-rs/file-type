use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27350220: FileFormat = FileFormat {
    id: 27_350_220,
    puid: "wikidata/27350220",
    name: "ADRG General Information File",
    extensions: &["gen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
