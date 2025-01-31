use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29904543: FileFormat = FileFormat {
    id: 29_904_543,
    puid: "wikidata/29904543",
    name: "Statistical Analysis System data set view",
    extensions: &["sas7bvew", "sv2", "sv7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
