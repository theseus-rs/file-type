use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124987792: FileFormat = FileFormat {
    id: 124_987_792,
    puid: "wikidata/124987792",
    name: "Dr.Geo document",
    extensions: &["fgeo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
