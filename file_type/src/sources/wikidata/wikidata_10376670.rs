use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10376670: FileFormat = FileFormat {
    id: 10_376_670,
    puid: "wikidata/10376670",
    name: "tar.bz2",
    extensions: &["tar.bz2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
