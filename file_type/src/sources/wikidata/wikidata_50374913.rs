use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50374913: FileFormat = FileFormat {
    id: 50_374_913,
    puid: "wikidata/50374913",
    name: "AutoCAD Design Web Format",
    extensions: &["dwfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
