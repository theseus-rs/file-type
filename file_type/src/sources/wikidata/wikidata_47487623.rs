use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47487623: FileFormat = FileFormat {
    id: 47_487_623,
    puid: "wikidata/47487623",
    name: "GEM Metafile",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
