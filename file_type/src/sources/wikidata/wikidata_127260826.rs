use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127260826: FileFormat = FileFormat {
    id: 127_260_826,
    puid: "wikidata/127260826",
    name: "Abaqus substructure file",
    extensions: &["sim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
