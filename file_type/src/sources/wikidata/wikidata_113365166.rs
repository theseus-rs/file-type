use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113365166: FileFormat = FileFormat {
    id: 113_365_166,
    puid: "wikidata/113365166",
    name: "Camtasia Recording",
    extensions: &["trec"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
