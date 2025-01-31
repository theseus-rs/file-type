use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111316260: FileFormat = FileFormat {
    id: 111_316_260,
    puid: "wikidata/111316260",
    name: "Sample Cell II Mac instrument",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
