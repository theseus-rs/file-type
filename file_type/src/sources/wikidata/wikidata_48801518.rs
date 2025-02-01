use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_48801518: FileFormat = FileFormat {
    id: 48_801_518,
    puid: "wikidata/48801518",
    name: "Adobe FrameMaker Document, version 8",
    extensions: &["fm"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
