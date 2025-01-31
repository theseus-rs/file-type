use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47538998: FileFormat = FileFormat {
    id: 47_538_998,
    puid: "wikidata/47538998",
    name: "AutoCAD Landscape Library",
    extensions: &["lli"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
