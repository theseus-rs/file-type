use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551294: FileFormat = FileFormat {
    id: 28_551_294,
    puid: "wikidata/28551294",
    name: "Adobe Color Table File",
    extensions: &["act"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
