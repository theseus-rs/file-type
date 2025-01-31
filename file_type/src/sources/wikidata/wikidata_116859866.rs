use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116859866: FileFormat = FileFormat {
    id: 116_859_866,
    puid: "wikidata/116859866",
    name: "Lesson File",
    extensions: &["lsn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
