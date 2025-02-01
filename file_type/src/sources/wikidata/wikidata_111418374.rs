use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111418374: FileFormat = FileFormat {
    id: 111_418_374,
    puid: "wikidata/111418374",
    name: "Adobe Bridge Cache File",
    extensions: &["bc", "bcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
