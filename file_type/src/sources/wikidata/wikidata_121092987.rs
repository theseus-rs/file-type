use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121092987: FileFormat = FileFormat {
    id: 121_092_987,
    puid: "wikidata/121092987",
    name: "Punch! 3D Object",
    extensions: &["pob"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
