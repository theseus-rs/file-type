use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60628025: FileFormat = FileFormat {
    id: 60_628_025,
    puid: "wikidata/60628025",
    name: "FoxPro Database, version 2.5",
    extensions: &["dbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
