use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2931409: FileFormat = FileFormat {
    id: 2_931_409,
    puid: "wikidata/2931409",
    name: "CFD General Notation System",
    extensions: &["cgns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
