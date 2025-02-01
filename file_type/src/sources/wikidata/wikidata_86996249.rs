use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86996249: FileFormat = FileFormat {
    id: 86_996_249,
    puid: "wikidata/86996249",
    name: "PaperPort MAX 3-4",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
