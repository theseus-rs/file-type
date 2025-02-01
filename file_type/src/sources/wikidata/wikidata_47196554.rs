use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47196554: FileFormat = FileFormat {
    id: 47_196_554,
    puid: "wikidata/47196554",
    name: "AppleWorks Painting file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
