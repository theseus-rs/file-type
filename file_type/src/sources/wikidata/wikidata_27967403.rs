use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967403: FileFormat = FileFormat {
    id: 27_967_403,
    source_type: SourceType::Wikidata,
    name: "CUD-FM-File",
    extensions: &["cff"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
