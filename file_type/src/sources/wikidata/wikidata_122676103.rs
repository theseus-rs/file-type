use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122676103: FileFormat = FileFormat {
    id: 122_676_103,
    puid: "wikidata/122676103",
    name: "JASC Brush File",
    extensions: &["jbr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
