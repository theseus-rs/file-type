use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34306495: FileFormat = FileFormat {
    id: 34_306_495,
    puid: "wikidata/34306495",
    name: "Scifer archive",
    extensions: &["sen"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
