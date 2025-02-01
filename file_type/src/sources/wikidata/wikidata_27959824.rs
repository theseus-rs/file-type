use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959824: FileFormat = FileFormat {
    id: 27_959_824,
    puid: "wikidata/27959824",
    name: "Ableton Skin File",
    extensions: &["ask"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
