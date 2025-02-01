use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_39185662: FileFormat = FileFormat {
    id: 39_185_662,
    puid: "wikidata/39185662",
    name: "AHK script",
    extensions: &["ahk"],
    media_types: &["text/x-autohotkey"],
    internal_signatures: &[],
    related_formats: &[],
};
