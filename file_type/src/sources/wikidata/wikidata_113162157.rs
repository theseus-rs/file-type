use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113162157: FileFormat = FileFormat {
    id: 113_162_157,
    puid: "wikidata/113162157",
    name: "MyAdvancedLabelDesigner File",
    extensions: &["mlb", "mld"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
