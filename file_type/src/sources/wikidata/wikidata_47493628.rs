use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47493628: FileFormat = FileFormat {
    id: 47_493_628,
    puid: "wikidata/47493628",
    name: "Adobe InDesign Document, version CS5",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
