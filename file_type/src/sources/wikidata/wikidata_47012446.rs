use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012446: FileFormat = FileFormat {
    id: 47_012_446,
    puid: "wikidata/47012446",
    name: "Microstation CAD Drawing file format",
    extensions: &["dgn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
