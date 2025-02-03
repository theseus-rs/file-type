use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_109673475: FileFormat = FileFormat {
    id: 109_673_475,
    source_type: SourceType::Wikidata,
    name: "Eudora Mailbox",
    extensions: &["mbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
