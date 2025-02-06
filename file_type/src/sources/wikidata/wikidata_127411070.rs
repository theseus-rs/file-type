use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127411070: FileFormat = FileFormat {
    id: 127_411_070,
    source_type: SourceType::Wikidata,
    name: "Nim source code file",
    extensions: &["nim", "nimrod"],
    media_types: &["text/x-nim"],
    signatures: &[],
    related_formats: &[],
};
