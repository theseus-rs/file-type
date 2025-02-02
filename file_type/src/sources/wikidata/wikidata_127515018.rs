use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127515018: FileFormat = FileFormat {
    id: 127_515_018,
    source_type: SourceType::Wikidata,
    name: "Typescript implementation file",
    extensions: &["ts"],
    media_types: &["application/x-typescript", "text/x-typescript"],
    internal_signatures: &[],
    related_formats: &[],
};
