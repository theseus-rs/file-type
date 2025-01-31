use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47490002: FileFormat = FileFormat {
    id: 47_490_002,
    puid: "wikidata/47490002",
    name: "Adobe FrameMaker Document, version 7",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
