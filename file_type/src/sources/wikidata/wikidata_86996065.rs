use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86996065: FileFormat = FileFormat {
    id: 86_996_065,
    puid: "wikidata/86996065",
    name: "PaperPort MAX 5-7",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
