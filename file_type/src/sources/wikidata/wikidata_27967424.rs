use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967424: FileFormat = FileFormat {
    id: 27_967_424,
    source_type: SourceType::Wikidata,
    name: "Amazon downloader file",
    extensions: &["amz"],
    media_types: &["audio/x-amzxml"],
    signatures: &[],
    related_formats: &[],
};
