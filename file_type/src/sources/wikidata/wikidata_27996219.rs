use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27996219: FileFormat = FileFormat {
    id: 27_996_219,
    source_type: SourceType::Wikidata,
    name: "Eudora Mailbox Table of Contents",
    extensions: &["toc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
