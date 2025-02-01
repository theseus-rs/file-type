use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206177: FileFormat = FileFormat {
    id: 28_206_177,
    puid: "wikidata/28206177",
    name: "GIMP Brush",
    extensions: &["gbr", "gpb"],
    media_types: &["image/x-gimp-gbr", "image/x-gimp-gbr"],
    internal_signatures: &[],
    related_formats: &[],
};
