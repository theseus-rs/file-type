use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122305824: FileFormat = FileFormat {
    id: 122_305_824,
    puid: "wikidata/122305824",
    name: "PressRelease Message",
    extensions: &["prm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
