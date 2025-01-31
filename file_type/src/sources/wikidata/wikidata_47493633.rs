use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47493633: FileFormat = FileFormat {
    id: 47_493_633,
    puid: "wikidata/47493633",
    name: "Adobe InDesign Document, version CS6",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
