use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757774: FileFormat = FileFormat {
    id: 28_757_774,
    puid: "wikidata/28757774",
    name: "GEXF",
    extensions: &["gexf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
