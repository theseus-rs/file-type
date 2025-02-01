use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48814699: FileFormat = FileFormat {
    id: 48_814_699,
    puid: "wikidata/48814699",
    name: "DesignCAD for Windows Drawing",
    extensions: &["dw2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
