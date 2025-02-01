use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7265434: FileFormat = FileFormat {
    id: 7_265_434,
    puid: "wikidata/7265434",
    name: "Quicken Financial Exchange",
    extensions: &["qfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
