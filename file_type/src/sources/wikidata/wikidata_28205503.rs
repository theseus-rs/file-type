use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205503: FileFormat = FileFormat {
    id: 28_205_503,
    puid: "wikidata/28205503",
    name: "GEM resource file",
    extensions: &["rsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
