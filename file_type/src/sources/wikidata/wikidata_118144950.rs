use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118144950: FileFormat = FileFormat {
    id: 118_144_950,
    puid: "wikidata/118144950",
    name: "Serenade Symphony File",
    extensions: &["sph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
