use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111594686: FileFormat = FileFormat {
    id: 111_594_686,
    puid: "wikidata/111594686",
    name: "Adobe InDesign Document, version 1",
    extensions: &["ind", "indd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
