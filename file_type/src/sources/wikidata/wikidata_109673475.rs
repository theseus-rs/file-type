use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109673475: FileFormat = FileFormat {
    id: 109_673_475,
    source_type: SourceType::Wikidata,
    name: "Eudora Mailbox",
    extensions: &["mbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
