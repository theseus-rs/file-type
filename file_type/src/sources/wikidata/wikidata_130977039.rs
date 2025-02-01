use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130977039: FileFormat = FileFormat {
    id: 130_977_039,
    puid: "wikidata/130977039",
    name: "Singularity definition file",
    extensions: &["Singularity", "def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
