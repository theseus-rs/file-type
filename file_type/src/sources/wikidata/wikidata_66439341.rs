use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66439341: FileFormat = FileFormat {
    id: 66_439_341,
    puid: "wikidata/66439341",
    name: "Volkswriter file format",
    extensions: &["vw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
