use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967424: FileFormat = FileFormat {
    id: 27_967_424,
    source_type: SourceType::Wikidata,
    name: "Amazon downloader file",
    extensions: &["amz"],
    media_types: &["audio/x-amzxml"],
    internal_signatures: &[],
    related_formats: &[],
};
