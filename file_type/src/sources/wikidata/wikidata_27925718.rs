use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27925718: FileFormat = FileFormat {
    id: 27_925_718,
    puid: "wikidata/27925718",
    name: "DTED Level 1 Gazetteer Key file",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
