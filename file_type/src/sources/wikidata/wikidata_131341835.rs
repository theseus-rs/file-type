use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131341835: FileFormat = FileFormat {
    id: 131_341_835,
    source_type: SourceType::Wikidata,
    name: "UrbiScript source code file",
    extensions: &["u"],
    media_types: &["application/x-urbiscript"],
    internal_signatures: &[],
    related_formats: &[],
};
