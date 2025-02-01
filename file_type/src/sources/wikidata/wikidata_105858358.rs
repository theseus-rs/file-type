use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858358: FileFormat = FileFormat {
    id: 105_858_358,
    puid: "wikidata/105858358",
    name: "IAR Embedded Workbench Project",
    extensions: &["ewp"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
