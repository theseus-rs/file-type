use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1124477: FileFormat = FileFormat {
    id: 1_124_477,
    puid: "wikidata/1124477",
    name: "Efficient XML Interchange",
    extensions: &["exi"],
    media_types: &["application/exi"],
    internal_signatures: &[],
    related_formats: &[],
};
