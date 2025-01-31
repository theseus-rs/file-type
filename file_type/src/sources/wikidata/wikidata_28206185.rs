use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206185: FileFormat = FileFormat {
    id: 28_206_185,
    puid: "wikidata/28206185",
    name: "GIMP Pattern",
    extensions: &["pat"],
    media_types: &["image/x-gimp-pat"],
    internal_signatures: &[],
    related_formats: &[],
};
