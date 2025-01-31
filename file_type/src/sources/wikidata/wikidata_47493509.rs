use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47493509: FileFormat = FileFormat {
    id: 47_493_509,
    puid: "wikidata/47493509",
    name: "Adobe InDesign Document, version CS2",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
