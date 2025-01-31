use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117869071: FileFormat = FileFormat {
    id: 117_869_071,
    puid: "wikidata/117869071",
    name: "Relisys file",
    extensions: &["tef"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
