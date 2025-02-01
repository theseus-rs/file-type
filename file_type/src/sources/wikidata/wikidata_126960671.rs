use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126960671: FileFormat = FileFormat {
    id: 126_960_671,
    puid: "wikidata/126960671",
    name: "Vala source file",
    extensions: &["vala", "vapi"],
    media_types: &["text/x-vala", "text/x-vala"],
    internal_signatures: &[],
    related_formats: &[],
};
