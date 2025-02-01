use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2713137: FileFormat = FileFormat {
    id: 2_713_137,
    puid: "wikidata/2713137",
    name: "Crystallographic Information File",
    extensions: &["cif"],
    media_types: &["chemical/x-cif"],
    internal_signatures: &[],
    related_formats: &[],
};
