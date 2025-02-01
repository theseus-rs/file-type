use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111496643: FileFormat = FileFormat {
    id: 111_496_643,
    puid: "wikidata/111496643",
    name: "Spectrum 512 Extended, version 2",
    extensions: &["spx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
