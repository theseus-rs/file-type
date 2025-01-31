use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117338265: FileFormat = FileFormat {
    id: 117_338_265,
    puid: "wikidata/117338265",
    name: "Corel Catalog",
    extensions: &["clc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
