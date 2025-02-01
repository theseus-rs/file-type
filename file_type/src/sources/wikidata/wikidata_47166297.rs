use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47166297: FileFormat = FileFormat {
    id: 47_166_297,
    puid: "wikidata/47166297",
    name: "ClarisWorks Word Processor file format",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
