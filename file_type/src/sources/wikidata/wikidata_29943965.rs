use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29943965: FileFormat = FileFormat {
    id: 29_943_965,
    puid: "wikidata/29943965",
    name: "Statistical Package for the Social Sciences portable data format",
    extensions: &["por"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
