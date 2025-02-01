use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867634: FileFormat = FileFormat {
    id: 105_867_634,
    puid: "wikidata/105867634",
    name: "Nastran input data",
    extensions: &["nas"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
