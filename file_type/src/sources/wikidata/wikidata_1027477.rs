use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1027477: FileFormat = FileFormat {
    id: 1_027_477,
    puid: "wikidata/1027477",
    name: "Caltech Intermediate Form",
    extensions: &["cif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
