use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131684360: FileFormat = FileFormat {
    id: 131_684_360,
    puid: "wikidata/131684360",
    name: "Sierra IO System file format",
    extensions: &["exdg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
