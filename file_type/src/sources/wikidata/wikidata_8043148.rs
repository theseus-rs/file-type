use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8043148: FileFormat = FileFormat {
    id: 8_043_148,
    puid: "wikidata/8043148",
    name: "Xara Flare",
    extensions: &["xar"],
    media_types: &["application/vnd.xara"],
    internal_signatures: &[],
    related_formats: &[],
};
