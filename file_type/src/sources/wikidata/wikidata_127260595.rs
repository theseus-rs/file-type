use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127260595: FileFormat = FileFormat {
    id: 127_260_595,
    puid: "wikidata/127260595",
    name: "Abaqus/CAE model database",
    extensions: &["cae"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
