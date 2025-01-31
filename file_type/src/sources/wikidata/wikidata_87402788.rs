use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_87402788: FileFormat = FileFormat {
    id: 87_402_788,
    puid: "wikidata/87402788",
    name: "AutoCAD Temporary File",
    extensions: &["ac$"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
