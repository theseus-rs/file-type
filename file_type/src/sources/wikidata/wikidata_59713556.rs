use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59713556: FileFormat = FileFormat {
    id: 59_713_556,
    puid: "wikidata/59713556",
    name: "Gaussian Input Data File",
    extensions: &["gjf"],
    media_types: &["chemical/x-gaussian-input"],
    internal_signatures: &[],
    related_formats: &[],
};
