use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99973071: FileFormat = FileFormat {
    id: 99_973_071,
    puid: "wikidata/99973071",
    name: "OmniPage Document 18",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
