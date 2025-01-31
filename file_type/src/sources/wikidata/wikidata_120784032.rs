use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120784032: FileFormat = FileFormat {
    id: 120_784_032,
    puid: "wikidata/120784032",
    name: "3-D TopoQuads 2.0 File",
    extensions: &["tq2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
