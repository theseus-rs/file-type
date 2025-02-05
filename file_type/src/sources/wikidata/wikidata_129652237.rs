use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129652237: FileFormat = FileFormat {
    id: 129_652_237,
    source_type: SourceType::Wikidata,
    name: "Igor Pro procedure file",
    extensions: &["ipf"],
    media_types: &["text/ipf"],
    signatures: &[],
    related_formats: &[],
};
