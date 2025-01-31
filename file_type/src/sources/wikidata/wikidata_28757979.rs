use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757979: FileFormat = FileFormat {
    id: 28_757_979,
    puid: "wikidata/28757979",
    name: "Windows Setup inf_loc file",
    extensions: &["inf_loc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
