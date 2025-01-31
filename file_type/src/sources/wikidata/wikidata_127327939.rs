use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127327939: FileFormat = FileFormat {
    id: 127_327_939,
    puid: "wikidata/127327939",
    name: "COBOL Source Code File",
    extensions: &["cbl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
