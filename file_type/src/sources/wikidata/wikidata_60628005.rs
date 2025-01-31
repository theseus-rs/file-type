use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60628005: FileFormat = FileFormat {
    id: 60_628_005,
    puid: "wikidata/60628005",
    name: "FoxPro Database, version 2",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
