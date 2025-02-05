use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129177480: FileFormat = FileFormat {
    id: 129_177_480,
    source_type: SourceType::Wikidata,
    name: "Fennel source code file",
    extensions: &["fnl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
