use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63106742: FileFormat = FileFormat {
    id: 63_106_742,
    puid: "wikidata/63106742",
    name: "Microsoft Works Word Processor file 3-4 for Windows",
    extensions: &["wps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
