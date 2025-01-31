use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50376371: FileFormat = FileFormat {
    id: 50_376_371,
    puid: "wikidata/50376371",
    name: "SHA256 File",
    extensions: &["sha256"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
