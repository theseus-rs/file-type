use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857670: FileFormat = FileFormat {
    id: 105_857_670,
    puid: "wikidata/105857670",
    name: "Citrix Independent Computer Architecture",
    extensions: &["ica"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
