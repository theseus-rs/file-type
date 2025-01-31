use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60558566: FileFormat = FileFormat {
    id: 60_558_566,
    puid: "wikidata/60558566",
    name: "ClarisWorks Word Processor",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
