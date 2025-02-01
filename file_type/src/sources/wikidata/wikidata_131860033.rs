use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131860033: FileFormat = FileFormat {
    id: 131_860_033,
    puid: "wikidata/131860033",
    name: "VPIC file format",
    extensions: &["vpc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
