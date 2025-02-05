use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131341835: FileFormat = FileFormat {
    id: 131_341_835,
    source_type: SourceType::Wikidata,
    name: "UrbiScript source code file",
    extensions: &["u"],
    media_types: &["application/x-urbiscript"],
    signatures: &[],
    related_formats: &[],
};
