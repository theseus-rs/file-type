use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100324042: FileFormat = FileFormat {
    id: 100_324_042,
    puid: "wikidata/100324042",
    name: "Corel Print House Document, version 1",
    extensions: &["cpd", "cph"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
