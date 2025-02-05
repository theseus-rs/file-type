use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127515018: FileFormat = FileFormat {
    id: 127_515_018,
    source_type: SourceType::Wikidata,
    name: "Typescript implementation file",
    extensions: &["ts"],
    media_types: &["application/x-typescript", "text/x-typescript"],
    signatures: &[],
    related_formats: &[],
};
