use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28530510: FileFormat = FileFormat {
    id: 28_530_510,
    puid: "wikidata/28530510",
    name: "Structure-data file",
    extensions: &["sdf"],
    media_types: &["chemical/x-mdl-sdfile"],
    internal_signatures: &[],
    related_formats: &[],
};
