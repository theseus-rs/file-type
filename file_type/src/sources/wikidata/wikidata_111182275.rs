use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111182275: FileFormat = FileFormat {
    id: 111_182_275,
    puid: "wikidata/111182275",
    name: "ActionScript Remote File",
    extensions: &["asr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
