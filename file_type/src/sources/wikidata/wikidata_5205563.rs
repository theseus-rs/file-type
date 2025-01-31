use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5205563: FileFormat = FileFormat {
    id: 5_205_563,
    puid: "wikidata/5205563",
    name: "Downloadable Sounds",
    extensions: &["dls"],
    media_types: &["audio/dls"],
    internal_signatures: &[],
    related_formats: &[],
};
