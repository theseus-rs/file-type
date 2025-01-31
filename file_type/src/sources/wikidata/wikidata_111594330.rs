use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111594330: FileFormat = FileFormat {
    id: 111_594_330,
    puid: "wikidata/111594330",
    name: "Adobe InDesign Library, version 4",
    extensions: &["indl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
