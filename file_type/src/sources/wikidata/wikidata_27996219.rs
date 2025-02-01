use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996219: FileFormat = FileFormat {
    id: 27_996_219,
    puid: "wikidata/27996219",
    name: "Eudora Mailbox Table of Contents",
    extensions: &["toc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
