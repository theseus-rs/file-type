use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777682: FileFormat = FileFormat {
    id: 28_777_682,
    puid: "wikidata/28777682",
    name: "mm",
    extensions: &["mm", "mm"],
    media_types: &["application/freemind", "application/x-freemind"],
    internal_signatures: &[],
    related_formats: &[],
};
