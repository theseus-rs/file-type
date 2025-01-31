use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66309247: FileFormat = FileFormat {
    id: 66_309_247,
    puid: "wikidata/66309247",
    name: "Access Database Runtime",
    extensions: &["accdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
