use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47493619: FileFormat = FileFormat {
    id: 47_493_619,
    puid: "wikidata/47493619",
    name: "Adobe InDesign Document, version CS4",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
