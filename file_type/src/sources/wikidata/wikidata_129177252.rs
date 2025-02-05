use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129177252: FileFormat = FileFormat {
    id: 129_177_252,
    source_type: SourceType::Wikidata,
    name: "Felix source code file",
    extensions: &["flx"],
    media_types: &["text/x-felix"],
    signatures: &[],
    related_formats: &[],
};
