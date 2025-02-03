use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979401: FileFormat = FileFormat {
    id: 27_979_401,
    source_type: SourceType::Wikidata,
    name: "JP2",
    extensions: &["jp2"],
    media_types: &["image/jp2"],
    internal_signatures: &[],
    related_formats: &[],
};
