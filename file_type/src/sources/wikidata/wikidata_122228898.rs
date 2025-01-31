use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122228898: FileFormat = FileFormat {
    id: 122_228_898,
    puid: "wikidata/122228898",
    name: "Oracle Password Hash",
    extensions: &["orc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
