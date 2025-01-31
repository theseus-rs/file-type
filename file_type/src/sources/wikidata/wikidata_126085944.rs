use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126085944: FileFormat = FileFormat {
    id: 126_085_944,
    puid: "wikidata/126085944",
    name: "IMF Package Asset Map",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
