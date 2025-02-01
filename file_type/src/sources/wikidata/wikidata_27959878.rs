use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959878: FileFormat = FileFormat {
    id: 27_959_878,
    puid: "wikidata/27959878",
    name: "Piston Collage song",
    extensions: &["ptcop", "pttune"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
