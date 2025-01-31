use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129414825: FileFormat = FileFormat {
    id: 129_414_825,
    puid: "wikidata/129414825",
    name: "Golo source code file",
    extensions: &["golo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
