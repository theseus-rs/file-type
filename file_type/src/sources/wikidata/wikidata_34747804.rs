use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34747804: FileFormat = FileFormat {
    id: 34_747_804,
    puid: "wikidata/34747804",
    name: "Supaplex Level format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
