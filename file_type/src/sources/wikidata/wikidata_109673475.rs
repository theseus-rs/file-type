use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109673475: FileFormat = FileFormat {
    id: 109_673_475,
    puid: "wikidata/109673475",
    name: "Eudora Mailbox",
    extensions: &["mbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
