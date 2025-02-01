use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51913144: FileFormat = FileFormat {
    id: 51_913_144,
    puid: "wikidata/51913144",
    name: "Instalit Script",
    extensions: &["pvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
