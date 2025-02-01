use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47167455: FileFormat = FileFormat {
    id: 47_167_455,
    puid: "wikidata/47167455",
    name: "ClarisWorks Database file format",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
