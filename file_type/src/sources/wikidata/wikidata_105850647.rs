use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850647: FileFormat = FileFormat {
    id: 105_850_647,
    puid: "wikidata/105850647",
    name: "Koda Form Designer Form",
    extensions: &["kxf"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
