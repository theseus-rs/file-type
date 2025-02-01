use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34747567: FileFormat = FileFormat {
    id: 34_747_567,
    puid: "wikidata/34747567",
    name: "Stronghold GM1",
    extensions: &["gm1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
