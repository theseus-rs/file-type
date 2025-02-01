use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71999956: FileFormat = FileFormat {
    id: 71_999_956,
    puid: "wikidata/71999956",
    name: "iTunes Cover Flow Data file format, version 2",
    extensions: &["itc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
