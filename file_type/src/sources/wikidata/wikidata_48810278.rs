use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48810278: FileFormat = FileFormat {
    id: 48_810_278,
    puid: "wikidata/48810278",
    name: "DesignCAD Drawing",
    extensions: &["dc", "dc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
