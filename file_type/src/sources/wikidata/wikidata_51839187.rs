use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51839187: FileFormat = FileFormat {
    id: 51_839_187,
    puid: "wikidata/51839187",
    name: "NAP Metafile",
    extensions: &["nap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
