use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99976195: FileFormat = FileFormat {
    id: 99_976_195,
    puid: "wikidata/99976195",
    name: "XDOMEA 2.0.1",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
