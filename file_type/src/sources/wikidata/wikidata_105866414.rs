use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866414: FileFormat = FileFormat {
    id: 105_866_414,
    puid: "wikidata/105866414",
    name: "Poser pose",
    extensions: &["pz2"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
