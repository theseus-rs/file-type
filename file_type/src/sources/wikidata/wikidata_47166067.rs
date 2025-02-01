use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47166067: FileFormat = FileFormat {
    id: 47_166_067,
    puid: "wikidata/47166067",
    name: "ClarisWorks file format version 2-3",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
