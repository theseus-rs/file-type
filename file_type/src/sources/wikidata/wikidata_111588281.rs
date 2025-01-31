use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111588281: FileFormat = FileFormat {
    id: 111_588_281,
    puid: "wikidata/111588281",
    name: "Adobe InDesign Library 2",
    extensions: &["indl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
