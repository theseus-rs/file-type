use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47489943: FileFormat = FileFormat {
    id: 47_489_943,
    puid: "wikidata/47489943",
    name: "Adobe FrameMaker Document, version 2",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
