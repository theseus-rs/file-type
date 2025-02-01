use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127260495: FileFormat = FileFormat {
    id: 127_260_495,
    puid: "wikidata/127260495",
    name: "Abaqus output database",
    extensions: &["odb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
