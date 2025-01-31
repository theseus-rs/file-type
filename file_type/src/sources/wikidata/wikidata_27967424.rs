use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967424: FileFormat = FileFormat {
    id: 27_967_424,
    puid: "wikidata/27967424",
    name: "Amazon downloader file",
    extensions: &["amz"],
    media_types: &["audio/x-amzxml"],
    internal_signatures: &[],
    related_formats: &[],
};
