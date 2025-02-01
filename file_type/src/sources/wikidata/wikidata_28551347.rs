use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551347: FileFormat = FileFormat {
    id: 28_551_347,
    puid: "wikidata/28551347",
    name: "Adobe Halftone Screens File",
    extensions: &["ahs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
