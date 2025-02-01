use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122305680: FileFormat = FileFormat {
    id: 122_305_680,
    puid: "wikidata/122305680",
    name: "PressRelease Project",
    extensions: &["prp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
