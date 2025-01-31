use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126960131: FileFormat = FileFormat {
    id: 126_960_131,
    puid: "wikidata/126960131",
    name: "Standard ML source code file",
    extensions: &["sml", "sml"],
    media_types: &["application/x-standardml", "text/x-standardml"],
    internal_signatures: &[],
    related_formats: &[],
};
