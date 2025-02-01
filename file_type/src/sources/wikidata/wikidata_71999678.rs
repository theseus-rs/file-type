use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71999678: FileFormat = FileFormat {
    id: 71_999_678,
    puid: "wikidata/71999678",
    name: "iTunes CoverFlow data file format",
    extensions: &["itc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
