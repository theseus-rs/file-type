use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67124021: FileFormat = FileFormat {
    id: 67_124_021,
    puid: "wikidata/67124021",
    name: "Print Artist greeting card file format",
    extensions: &["gc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
