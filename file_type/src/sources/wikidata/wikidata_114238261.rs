use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114238261: FileFormat = FileFormat {
    id: 114_238_261,
    puid: "wikidata/114238261",
    name: "Streaming Audio Metafile",
    extensions: &["lam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
