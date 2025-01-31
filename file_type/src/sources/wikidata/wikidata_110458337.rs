use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110458337: FileFormat = FileFormat {
    id: 110_458_337,
    puid: "wikidata/110458337",
    name: "SpritePad Image Format",
    extensions: &["spd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
