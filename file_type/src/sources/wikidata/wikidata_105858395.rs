use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858395: FileFormat = FileFormat {
    id: 105_858_395,
    puid: "wikidata/105858395",
    name: "EAGLE script",
    extensions: &["ulp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
