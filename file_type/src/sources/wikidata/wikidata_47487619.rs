use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47487619: FileFormat = FileFormat {
    id: 47_487_619,
    puid: "wikidata/47487619",
    name: "GEM Metafile",
    extensions: &["gem"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
